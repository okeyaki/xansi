extern crate rstest;
extern crate speculate;

use rstest::rstest;
use speculate::speculate;

speculate! {
    describe "#remove_ansi_escape_sequences" {
        #[rstest(source, expected,
            case("", ""),
            case("\x1b[1;37m", ""),
            case("\x1b[0m", ""),
            case("\x1b[40m", ""),
        )]
        fn can_remove_ansi_escape_sequences(source: String, expected: String) {
            assert_eq!(xansi::remove_ansi_escape_sequences(&source), expected);
        }
    }
}
