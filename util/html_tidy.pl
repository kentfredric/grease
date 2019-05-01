#!perl
use strict;
use warnings;

## A fucking crude HTML re-flow tool because the internet is shit,
## minified HTML is diff-unfriendly and all the other shit uses horrible shite,
## doesn't work, or is stupidly tripped up by "invalid" formatting that is produced by
## rust (despite working fine in browsers), and "fixing" it for you, emitting a steaming
## heap of broken HTML that doesn't work.
##
## This code is intentionally very naive, parses HTML with regex (OMG!), but all its gonna
## do is insert "\n" in a few places so "git diff" isn't an entire shit-show for every goddamn change.
##
## This is _incredibly_ crude as it relies on the assumption a human will vet the output
## _AND_ tweak this code as need be, thus, if you borrow this yourself and try it, your warranty is
## void, don't come crying to me.
##
## Find something that works, write something that works, _THEN_ talk to me.

for my $file (@ARGV) {
    my $dest_file = $file . ".__new";

    open my $fh, '<', $file or die "Can't open $file, $!";
    my $html = do { local $/; <> };
    close $fh or die "Can't close $file, $!";

    # Inject \n above and below these self-closing tags
    for my $i (qw( !DOCTYPE html head meta link )) {
        $html =~ s/(<\Q$i\E[^>]+>)/\n$1\n/msg;
    }

    # Inject \n "around" these tags
    for my $i (
        qw( html head div pre noscript style script body nav form h1 h2 h3 h4 h5 section )
      )
    {
        $html =~ s/(<\Q$i\E[^>]*>)/\n$1/msg;
        $html =~ s/(<\/\Q$i\E[^>]*>)/$1\n/msg;
    }

    # Inject \n "inside" these tags
    for my $i (qw( head div noscript body nav form section )) {
        $html =~ s/(<\Q$i\E[^>]*>)/$1\n/msg;
        $html =~ s/(<\/\Q$i\E[^>]*>)/\n$1/msg;
    }

    # Make sure comments get their own line at least
    $html =~ s/(<!--.*?-->)/$1\n/msg;

    # Above code adds a shitload of duplicate newlines, so dedupe them.
    $html =~ s/\n\n+/\n/msg;

    open my $dfh, ">", $dest_file or die "Can't open $dest_file, $!";
    $dfh->print($html) or die "Error writing to $dest_file, $!";
    close $dfh or die "Cannot close $dest_file, $!";

    rename "$dest_file", "$file"
      or die "Error renaming $dest_file to $file, $!";
}
