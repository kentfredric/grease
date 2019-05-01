#!perl
use strict;
use warnings;
use feature 'fc';
use File::Temp qw( tempfile );

my %feature_defaults = (
    'herringbone' => 0,
    test          => 1,
    lint          => 0,
    permissive    => 0,
);

my %branches = (

    # Branch to derive
    source => 'clean',

    # Branch to derive to
    dest => 'rustdoc/clean',

    # Branch essential build/lint/test/derive tools are found in
    tools => 'clean',
);

my %whitelists = (
    sources => [
        qw(
          src tests .gitignore Cargo.lock Cargo.toml rustfmt.toml benches grease-bin grease
          util/rustflags.sh
          )
    ],
    tools => [
        qw(
          util
          )
    ],
);

my (@poststrip) = (
    qw(
      util/
      )
);

if ( $ENV{SOURCE} ) {
    $branches{source} = $ENV{SOURCE};
    $branches{dest}   = 'rustdoc/' . $ENV{SOURCE};
}
if ( $ENV{DEST} ) {
    $branches{dest} = $ENV{DEST};
}

## Generates a derived branch
my $source_branch = $branches{source};
my $dest_branch   = $branches{dest};

my $original     = get_orig_graph();
my $parent_trans = {};

system( 'git', 'branch', '-M', $dest_branch, "${dest_branch}.old" );
system( 'git', 'checkout', '-f', '-l', '--orphan', $dest_branch ) == 0
  or die "Can't invoke checkout, $!, $?";

my $loop_step = 0;
my $n_steps   = scalar @{ $original->{order} };
while ( my $dest = pop @{ $original->{order} } ) {
    my $info = $original->{info}->{$dest};
    $loop_step++;
    warn sprintf qq{\e[1m===[ %d of %d : \e[0m %s \e[1m]===\e[0m\n},
      $loop_step, $n_steps,
      $dest;

    #  last if $loop_step > 3;
    if ( my (@files) = current_files() ) {
        system( 'git', 'rm', '-q', '-f', @files ) == 0
          or die "Can't git rm, $! $?";
    }
    for my $file ( current_files("-o") ) {
        next if $file =~ /\A(.md5_cache|target)\//;
        warn sprintf qq{\e[1;33mrm \e[0m%s \e[0m\n}, $file;

        unlink $file or die "Can't unlink $file";
    }

    for my $target ( @{ $whitelists{tools} } ) {
        if ( commit_has_file( $branches{tools}, $target ) ) {

            # warn sprintf qq{\e[1;32m===> \e[0m%s \e[1;32mvia\e[0m %s\n},
            #    $target, $branches{tools};

            system( 'git', 'checkout', $branches{tools}, '--', $target ) == 0
              or die "Can't restore tool to $branches{tools} : $target , $! $?";

        }
        else {
            # warn sprintf
            #    qq{\e[1;32m===> \e[0m%s \e[1;31mnot in\e[0m %s (%s)\n},
            #    $target, $branches{tools}, "$! $?";
        }
    }
    for my $target ( @{ $whitelists{sources} } ) {
        if ( commit_has_file( $dest, $target ) ) {

            # warn sprintf qq{\e[1;33m===> \e[0m%s \e[1;33mvia\e[0m %s\n},
            #    $target, $dest;

            system( 'git', 'checkout', $dest, '--', $target ) == 0
              or die "Can't restore to $dest : $target , $! $?";

        }
        else {
         #  warn sprintf qq{\e[1;33m===> \e[0m%s \e[1;31mnot in\e[0m %s (%s)\n},
         #    $target, $dest, "$! $?";
        }
    }

    if ( want_feature('lint') ) {
        system( "bash", "util/lint.sh" ) == 0 or die "lint failed!, $! $?";
    }
    if ( want_feature('test') ) {
        system( "bash", "util/test.sh" ) == 0 or die "lint failed!, $! $?";
    }

    system( 'mkdir', '-p', 'target/doc/' ) == 0 or die "Can't mkdir, $! $?";
    if ( system( "bash", "util/gendoc.sh" ) != 0 ) {
        if ( want_feature('permissive') ) {
            warn "Error running gendoc, $!, $?";
        }
        else {
            die "Error running gendoc, $!, $?";
        }
    }
    if ( system( "git", "add", "-f", "target/doc" ) != 0 ) {

        if ( want_feature('permissive') ) {
            warn "Can't add generated doc tree, $!, $?";
        }
        else {
            die "Can't add generated doc tree, $!, $?";
        }
    }
    for my $path (@poststrip) {
        for my $file ( grep { /\A\Q$path\E/ } current_files() ) {
            system( 'git', 'rm', '-q', '-f', $file ) == 0
              or die "Can't git rm $file, $! $?";
        }
    }
    system( "touch", "target/doc/.nojekyll" );
    system( "git", "add", "-f", "target/doc/.nojekyll" );

    local $ENV{GIT_AUTHOR_NAME}  = $info->{aname};
    local $ENV{GIT_AUTHOR_DATE}  = adj_ts( $info->{adate} );
    local $ENV{GIT_AUTHOR_EMAIL} = $info->{aemail};

    local $ENV{GIT_COMMITTER_NAME}  = $info->{cname};
    local $ENV{GIT_COMMITTER_DATE}  = adj_ts( $info->{cdate} );
    local $ENV{GIT_COMMITTER_EMAIL} = $info->{cemail};

    system( 'git', '--no-pager', 'diff', '--summary', '--stat', '--cached' );
    printf "\e[37;1m%s\e[0m\n", $info->{subject};

    my $tempfile = File::Temp->new( TEMPLATE => "gen_derive-XXXXXXX" );
    printf {$tempfile} '[derived=%s] ', $info->{abbrev};
    print {$tempfile} $info->{body};
    close $tempfile;

    my $treeish = do {
        open my $fh, '-|', 'git', 'write-tree'
          or die "Can't open git, $? $!";
        local $/ = undef;
        my $ret = scalar <$fh>;
        close $fh or die "Error in git write-tree, $! $?";
        $ret =~ s/\s*\z//ms;
        $ret;
    };

    # warn "tree: $treeish\n";

    my (@parent_extras);
    if ( exists $original->{parents}->{$dest} ) {
        for my $parent ( @{ $original->{parents}->{$dest} } ) {
            if ( not exists $parent_trans->{$parent} ) {
                die "No parent translation for $parent";
            }
            push @parent_extras, "-p", $parent_trans->{$parent};
        }
    }
    if ( want_feature("herringbone") ) {
        push @parent_extras, "-p", $dest;
    }

    # warn "parents: @parent_extras\n";

    my $commitish = do {
        open my $fh, '-|', 'git', 'commit-tree', @parent_extras, '-F',
          $tempfile->filename, $treeish
          or die "Can't git commit-tree, $? $!";
        local $/ = undef;
        my $ret = scalar <$fh>;
        close $fh or die "Error in git commit-tree, $! $?";
        $ret =~ s/\s*\z//ms;
        $ret;
    };
    $parent_trans->{$dest} = $commitish;

    # warn "commit: $dest -> $commitish\n";
    system( "git", "update-ref", "-m", "Generating from $dest",
        "HEAD", $commitish ) == 0
      or die "Head update failed, $! $?";
}

sub get_orig_graph {

    my %fmt_spec = (
        'abbrev'  => '%h',
        'parents' => '%P',
        'aname'   => '%an',
        'aemail'  => '%ae',
        'adate'   => '%ad',
        'cname'   => '%cn',
        'cemail'  => '%ce',
        'cdate'   => '%cd',
        'subject' => '%s',
        'body'    => '%B',
    );
    my $fmt = join q[], map { sprintf "%s: %s%s", $_ => $fmt_spec{$_}, '%x00' }
      sort keys %fmt_spec;
    $fmt .= '%x1E';

    open my $fh, '-|', 'git', 'rev-list', '--date=raw', '--abbrev=6',
      '--pretty=format:' . $fmt, $source_branch
      or die "Can't invoke git rev-list, $! $?";
    local $/ = "\n";
    my (@ref_order);
    my (%parents);
    my (%commit_info);
    while ( my $line = <$fh> ) {
        chomp $line;
        if ( $line =~ /\Acommit (\S+)\z/ ) {
            my $commit = "$1";
            push @ref_order, $commit;
            my $data = do {
                local $/ = "\x1E";
                my $internal = scalar <$fh>;
                chomp $internal;
                $internal;
            };
            my %records = map { split /: /, $_, 2 }
              split "\0", $data;

            $commit_info{$commit} = \%records;
            if ( length $records{'parents'} ) {
                $parents{$commit} = [ split /\s+/, $records{'parents'} ];
            }
        }
    }
    return {
        parents => \%parents,
        order   => \@ref_order,
        info    => \%commit_info,
    };
}

sub want_feature {
    my ( $name, ) = @_;
    my $default   = $feature_defaults{$name};
    my $want      = fc("$name");
    my $dontwant  = fc("-$name");

    for my $token ( map fc, split /\s+/,
        ( defined $ENV{DERIVE} ? $ENV{DERIVE} : '' ) )
    {
        if ( $token eq $want ) {
            $default = 1;
            next;
        }
        if ( $token eq $dontwant ) {
            $default = 0;
            next;
        }
    }
    return $default;
}

use Capture::Tiny qw( capture_stdout );

sub commit_has_file {
    my ( $commit, $file ) = @_;
    my $stdout = capture_stdout {
        system( 'git', 'rev-parse', '-q', '--verify', sprintf q[%s:%s],
            $commit, $file );
    };
    if ( $stdout =~ /\A\s*\z/ms ) {
        return;
    }
    $stdout =~ s/\s*\z//ms;
    return $stdout;
}

sub current_files {
    my (@xargs) = @_;
    open my $fh, '-|', 'git', 'ls-files', '-z', @xargs
      or die "Can't run git ls-files, $! $?";
    local $/ = "\0";
    my @lines = <$fh>;
    close $fh or die "git ls-files errored: $? $!";
    chomp for @lines;
    @lines;
}

sub adj_ts {
    my ( $time, $zone ) = split / /, $_[0];
    sprintf "%s %s", $time + 1, $zone;
}
