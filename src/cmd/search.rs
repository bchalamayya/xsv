use regex::RegexBuilder;

use CliResult;
use config::{Config, Delimiter};
use select::SelectColumns;
use util;

static USAGE: &'static str = "
Filters CSV data by whether the given regex matches a row.

The regex is applied to each field in each row, and if any field matches,
then the row is written to the output. The columns to search can be limited
with the '--select' flag (but the full row is still written to the output if
there is a match).

Usage:
    xsv search [options] <regex> [<input>]
    xsv search --help

search options:
    -i, --ignore-case      Case insensitive search. This is equivalent to
                           prefixing the regex with '(?i)'.
    -s, --select <arg>     Select the columns to search. See 'xsv select -h'
                           for the full syntax.
    -v, --invert-match     Select only rows that did not match

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. (i.e., They are not searched, analyzed,
                           sliced, etc.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
";

#[derive(RustcDecodable)]
struct Args {
    arg_input: Option<String>,
    arg_regex: String,
    flag_select: SelectColumns,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_invert_match: bool,
    flag_ignore_case: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = try!(util::get_args(USAGE, argv));
    let pattern = try!(
        RegexBuilder::new(&*args.arg_regex)
                     .case_insensitive(args.flag_ignore_case)
                     .compile()
    );
    let rconfig = Config::new(&args.arg_input)
                         .delimiter(args.flag_delimiter)
                         .no_headers(args.flag_no_headers)
                         .select(args.flag_select);

    let mut rdr = try!(rconfig.reader());
    let mut wtr = try!(Config::new(&args.flag_output).writer());

    let headers = try!(rdr.byte_headers());
    let nsel = try!(rconfig.normal_selection(&*headers));

    if !rconfig.no_headers {
        try!(wtr.write(headers.iter()));
    }
    for row in rdr.records() {
        let row = try!(row);
        let mut m = nsel.select(row.iter()).any(|f| pattern.is_match(&**f));
        if args.flag_invert_match {
            m = !m;
        }
        if m {
            try!(wtr.write(row.iter().map(|f| &**f)));
        }
    }
    Ok(try!(wtr.flush()))
}
