#!perl
use strict;
use warnings;

## A fucking crude CSS re-flow tool because the internet is shit,
## minified CSS is diff-unfriendly and all the other shit uses horrible shite
## like:
##    npm and a mountain of deps which is a god fucking mess to install
##    ruby, but old versions of ruby which no longer exist
##    php, and also not updated for CSS3 shit so it breaks
##    C++, but the source code isn't available and I'm not downloading a binary from sourceforge
##    python, but not in a way anybody can actually use it
## So, yeah, fuck people.
## This is _incredibly_ crude as it relies on the assumption a human will vet the output
## _AND_ tweak this code as need be, thus, if you borrow this yourself and try it, your warranty is
## void, don't come crying to me.

BEGIN {
    *debug =
        $ENV{DEBUG}
      ? sub(&) { *stderr->printf( "\e[1m%s\e[0m\n", $_[0]->() ) }
      : sub(&) { }
}

for my $file (@ARGV) {
    debug { "Opening $file" };
    open my $fh, '<', $file or die "Can't open $file, $!";
    debug { "Opening $file.__new" };
    open my $ofh, '>', "$file.__new" or die "Can't open $file.__new, $!";
    my $content = do { local $/; <> };
    close $fh or die "Can't close $file, $!";
    debug { "Done reading $file" };
    while ( defined( my $unit = next_unit($content) ) ) {
        debug { sprintf "read: %s, todo: %s", length $unit, length $content };
        $ofh->printf( "%s\n", $unit ) or die "Can't write to $file.__new, $!";
        substr $content, 0, length $unit, "";
    }
    close $ofh or die "Can't close $file.__new, $!";
    rename "$file.__new", "$file" or die "Can't rename $file.__new to $file $!";
}

sub next_unit {
    my ($input)        = @_;
    my $body           = "";
    my $closing_tokens = 0;

    return if not length $input;
    if ( $input =~ /\A([^{]+?\{)/ ) {
        $body .= "$1";
        $closing_tokens++;
        substr $input, 0, length "$1", "";
    }
    if ( $input =~ /\A(\s*)\z/ ) {
        return "$1";
    }
    while ( $closing_tokens > 0 ) {
        if ( $input =~ /\A([^{}]+\{)/ ) {
            $body .= "$1";
            $closing_tokens++;
            substr $input, 0, length "$1", "";
            next;
        }
        if ( $input =~ /\A([^{}]*\})/ ) {
            $body .= "$1";
            $closing_tokens--;
            substr $input, 0, length "$1", "";
            next;
        }
        die;
    }
    return $body;
}
