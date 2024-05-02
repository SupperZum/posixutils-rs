//
// Copyright (c) 2024 Jeff Garzik
// Copyright (c) 2024 Hemi Labs, Inc.
//
// This file is part of the posixutils-rs project covered under
// the MIT License.  For the full license text, please see the LICENSE
// file in the root directory of this project.
// SPDX-License-Identifier: MIT
//

use chrono::{DateTime, Local};
use plib::{run_test, run_test_with_checker, TestPlan};
use regex::Regex;
use std::fs;
use std::io::Read;
const PR_DATE_TIME_FORMAT: &str = "%b %d %H:%M %Y";

fn expand_test_noargs(test_data: &str, expected_output: &str) {
    run_test(TestPlan {
        cmd: String::from("expand"),
        args: Vec::new(),
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn head_test(test_data: &str, expected_output: &str) {
    run_test(TestPlan {
        cmd: String::from("head"),
        args: Vec::new(),
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn wc_test(args: &[&str], test_data: &str, expected_output: &str) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("wc"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn csplit_test(args: &[&str], test_data: &str, expected_output: &str) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("csplit"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn nl_test(args: &[&str], test_data: &str, expected_output: &str) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("nl"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn pr_test(args: &[&str], test_data: &str, expected_output: &str) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("pr"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn cut_test(args: &[&str], test_data: &str, expected_output: &str) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("cut"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn pr_read_test_file(
    output_filename: &str,
    input_filename: &str,
    header: Option<&str>,
    date: Option<String>,
) -> String {
    let re = Regex::new(r"<DATE>|<FILENAME>").unwrap();

    let dt_string = date.unwrap_or_else(|| {
        let metadata = fs::metadata(input_filename).unwrap();
        let last_modified_time = metadata.modified().unwrap();
        let dt: DateTime<Local> = last_modified_time.into();
        dt.format(PR_DATE_TIME_FORMAT).to_string()
    });

    let mut file = fs::File::open(output_filename).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let s = re.replace_all(&buf, |captures: &regex::Captures<'_>| -> String {
        let marker = captures.get(0).unwrap();
        match marker.as_str() {
            "<DATE>" => dt_string.clone(),
            "<FILENAME>" => header.unwrap_or(input_filename).to_string(),
            _ => panic!("Unknown pattern"),
        }
    });

    s.to_string()
}

#[test]
fn test_expand_basic() {
    expand_test_noargs("", "");
    expand_test_noargs("a\tb\tc\n", "a       b       c\n");
}

#[test]
fn test_head_basic() {
    head_test("a\nb\nc\nd\n", "a\nb\nc\nd\n");
    head_test(
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n0\n",
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n0\n",
    );
    head_test(
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n0\na\n",
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n0\n",
    );
}

#[test]
fn test_wc_empty() {
    wc_test(&["-c"], "", "0\n");
    wc_test(&["-l"], "", "0\n");
    wc_test(&["-w"], "", "0\n");
}

#[test]
fn test_wc_one() {
    wc_test(&["-c"], "x", "1\n");
    wc_test(&["-l"], "x", "0\n");
    wc_test(&["-w"], "x", "1\n");
}

#[test]
fn test_wc_two() {
    wc_test(&["-c"], "x y\n", "4\n");
    wc_test(&["-l"], "x y\n", "1\n");
    wc_test(&["-w"], "x y\n", "2\n");
}

#[test]
fn test_csplit_text_by_lines() {
    csplit_test(
        &["-f", "text", "-", "5", "{3}"],
        "1sdfghnm
2sadsgdhjmf
3zcxbncvm vbm
4asdbncv
5adsbfdgfnfm
6sdfcvncbmcg
7zsdgdgfndcgmncg
8asdbsfdndcgmn
9sfbdxgfndcgmncgmn
10dvsd
11
12
13
14
15
16
17",
        "43\n\n57\n\n31\n\n14\n\n",
    );
    std::fs::remove_file("text00").unwrap();
    std::fs::remove_file("text01").unwrap();
    std::fs::remove_file("text02").unwrap();
    std::fs::remove_file("text03").unwrap();
}

#[test]
fn test_csplit_text_by_lines_from_file() {
    csplit_test(
        &["-f", "text_f", "tests/assets/test_file.txt", "5", "{3}"],
        "",
        "43\n\n57\n\n31\n\n14\n\n",
    );
    std::fs::remove_file("text_f00").unwrap();
    std::fs::remove_file("text_f01").unwrap();
    std::fs::remove_file("text_f02").unwrap();
    std::fs::remove_file("text_f03").unwrap();
}

#[test]
fn test_csplit_c_code_by_regex() {
    csplit_test(
        &[
            "-f",
            "code_c",
            "tests/assets/test_file_c",
            r"%main\(%",
            "/^}/+1",
            "{3}",
        ],
        "",
        "59\n\n53\n\n53\n\n54\n\n",
    );
    std::fs::remove_file("code_c00").unwrap();
    std::fs::remove_file("code_c01").unwrap();
    std::fs::remove_file("code_c02").unwrap();
    std::fs::remove_file("code_c03").unwrap();
}

#[test]
fn test_csplit_c_code_by_regex_negative_offset() {
    csplit_test(
        &[
            "-f",
            "code_c_neg",
            "tests/assets/test_file_c",
            r"%main\(%",
            "/^}/-2",
            "{3}",
        ],
        "",
        "12\n\n46\n\n52\n\n107\n\n",
    );
    std::fs::remove_file("code_c_neg00").unwrap();
    std::fs::remove_file("code_c_neg01").unwrap();
    std::fs::remove_file("code_c_neg02").unwrap();
    std::fs::remove_file("code_c_neg03").unwrap();
}

#[test]
fn test_csplit_c_code_by_regex_suppress() {
    csplit_test(
        &[
            "-s",
            "-f",
            "code_c_s",
            "tests/assets/test_file_c",
            r"%main\(%",
            "/^}/+1",
            "{3}",
        ],
        "",
        "",
    );
    std::fs::remove_file("code_c_s00").unwrap();
    std::fs::remove_file("code_c_s01").unwrap();
    std::fs::remove_file("code_c_s02").unwrap();
    std::fs::remove_file("code_c_s03").unwrap();
}

#[test]
fn test_csplit_c_code_by_regex_with_number() {
    csplit_test(
        &[
            "-f",
            "code_c_n",
            "-n",
            "3",
            "tests/assets/test_file_c",
            r"%main\(%",
            "/^}/+1",
            "{3}",
        ],
        "",
        "59\n\n53\n\n53\n\n54\n\n",
    );
    std::fs::remove_file("code_c_n000").unwrap();
    std::fs::remove_file("code_c_n001").unwrap();
    std::fs::remove_file("code_c_n002").unwrap();
    std::fs::remove_file("code_c_n003").unwrap();
}

#[test]
fn test_csplit_regex_by_empty_lines() {
    csplit_test(
        &["-f", "empty_lines", "tests/assets/empty_line.txt", "/^$/"],
        "",
        "6\n\n7\n\n",
    );
    std::fs::remove_file("empty_lines00").unwrap();
    std::fs::remove_file("empty_lines01").unwrap();
}

#[test]
fn test_csplit_regex_would_infloop() {
    csplit_test(
        &[
            "-f",
            "would_infloop",
            "tests/assets/would_infloop.txt",
            "/a/-1",
            "{*}",
        ],
        "",
        "2\n\n",
    );
    std::fs::remove_file("would_infloop00").unwrap();
}

#[test]
fn test_csplit_regex_in_uniq() {
    csplit_test(
        &["-f", "in_uniq", "tests/assets/in_uniq", "/^$/", "{*}"],
        "",
        "6\n\n10\n\n8\n\n9\n\n",
    );
    std::fs::remove_file("in_uniq00").unwrap();
    std::fs::remove_file("in_uniq01").unwrap();
    std::fs::remove_file("in_uniq02").unwrap();
    std::fs::remove_file("in_uniq03").unwrap();
}

#[test]
fn test_csplit_regex_in_uniq_2() {
    csplit_test(
        &["-f", "in_uniq_2_", "tests/assets/in_uniq", "/^$/-1", "{*}"],
        "",
        "3\n\n10\n\n8\n\n12\n\n",
    );
    std::fs::remove_file("in_uniq_2_00").unwrap();
    std::fs::remove_file("in_uniq_2_01").unwrap();
    std::fs::remove_file("in_uniq_2_02").unwrap();
    std::fs::remove_file("in_uniq_2_03").unwrap();
}

#[test]
fn test_csplit_regex_in_uniq_3() {
    csplit_test(
        &["-f", "in_uniq_3_", "tests/assets/in_uniq", "/^$/1", "{*}"],
        "",
        "7\n\n10\n\n8\n\n8\n\n",
    );
    std::fs::remove_file("in_uniq_3_00").unwrap();
    std::fs::remove_file("in_uniq_3_01").unwrap();
    std::fs::remove_file("in_uniq_3_02").unwrap();
    std::fs::remove_file("in_uniq_3_03").unwrap();
}

#[test]
fn test_csplit_regex_in_seq() {
    csplit_test(
        &["-f", "in_seq", "tests/assets/in_seq", "/2/", "/4/", "/6/"],
        "",
        "1\n\n3\n\n3\n\n1\n\n",
    );
    std::fs::remove_file("in_seq00").unwrap();
    std::fs::remove_file("in_seq01").unwrap();
    std::fs::remove_file("in_seq02").unwrap();
    std::fs::remove_file("in_seq03").unwrap();
}

#[test]
fn test_nl_justification() {
    nl_test(&["-n", "ln"], "a", "1     \ta\n");
    nl_test(&["-n", "rn"], "b", "     1\tb\n");
    nl_test(&["-n", "rz"], "c", "000001\tc\n");
}

#[test]
fn test_nl_newlines_at_end() {
    nl_test(&[], "a\n\n", "     1\ta\n       \n");
}

#[test]
fn test_nl_starting_number() {
    nl_test(&["-v", "2"], "a", "     2\ta\n");
}

#[test]
fn test_nl_number_increment() {
    let input = "\\:\\:\\:\nheader\n\\:\\:\nbody\n\\:\nfooter";
    // Without -p, the counter resets on delimiters
    nl_test(
        &["-h", "a", "-f", "a"],
        input,
        "\n     1\theader\n\n     1\tbody\n\n     1\tfooter\n",
    );

    // With -p, the counter increments even when encountering delimiters
    nl_test(
        &["-h", "a", "-f", "a", "-p"],
        input,
        "\n     1\theader\n\n     2\tbody\n\n     3\tfooter\n",
    );

    nl_test(
        &["-h", "a", "-f", "a", "-p", "-i", "2"],
        input,
        "\n     1\theader\n\n     3\tbody\n\n     5\tfooter\n",
    );
}

#[test]
fn test_nl_delimiter() {
    // Single character delimiter should be appended with the default second
    // character, ':'
    nl_test(
        &["-h", "a", "-f", "a", "-d", "?"],
        "?:?:?:\nheader\n?:?:\nbody\n?:\nfooter",
        "\n     1\theader\n\n     1\tbody\n\n     1\tfooter\n",
    );

    nl_test(
        &["-h", "a", "-f", "a", "-d", "?!"],
        "?!?!?!\nheader\n?!?!\nbody\n?!\nfooter",
        "\n     1\theader\n\n     1\tbody\n\n     1\tfooter\n",
    );
}

#[test]
fn test_nl_regex() {
    // NOTE: The implementation has better regex support than the reference.
    // `nl -b p.+ng` would fail to match the words ending with "ng" in the
    // original whereas it would in this Rust implementation. Might be
    // considered a bug?
    nl_test(
        &["-b", "p.*ng"],
        "something\nanything\neverything\ncat\ndog",
        "     1\tsomething\n     2\tanything\n     3\teverything\n       cat\n       dog\n",
    );
}

#[test]
fn test_pr_single_column() {
    let input = "tests/pr/lorem_ipsum.txt";
    let output = pr_read_test_file(
        "tests/pr/lorem_ipsum_output_single_column.txt",
        input,
        None,
        None,
    );
    pr_test(&[&input], "", &output);
}

#[test]
fn test_pr_multi_column() {
    let input = "tests/pr/lorem_ipsum.txt";
    let output = pr_read_test_file("tests/pr/lorem_ipsum_output_9_cols.txt", input, None, None);
    pr_test(&["-9", &input], "", &output);
}

#[test]
fn test_pr_multi_column_across() {
    let input = "tests/pr/lorem_ipsum.txt";
    let output = pr_read_test_file(
        "tests/pr/lorem_ipsum_output_2_cols_across.txt",
        input,
        None,
        None,
    );
    pr_test(&["-2", "-a", &input], "", &output);
}

#[test]
fn test_pr_multi_column_merge() {
    // This test requires the current timestamp.
    //
    // It's possible to inject the current timestamp to the expected output
    // before calling `pr_test` but that would cause spurious errors when the
    // minute portion changes in between now and when the process is actually
    // ran:
    //
    // Apr 18 14:12 2024
    // Apr 18 14:13 2024

    let input = "tests/pr/lorem_ipsum.txt";
    let args = &["+1:1", "-m", &input, &input, &input];
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    let test_plan = TestPlan {
        cmd: String::from("pr"),
        args: str_args,
        stdin_data: String::from(""),
        expected_out: String::from(""),
        expected_err: String::from(""),
        expected_exit_code: 0,
    };

    run_test_with_checker(test_plan, |_, output| {
        let stdout = String::from_utf8_lossy(&output.stdout);

        // MMM++++++++++YYYY
        let re = Regex::new(r"\w{3}.+\d{4}").unwrap();
        let captures = re.captures(&stdout).unwrap();
        let date = captures.get(0).unwrap().as_str();

        let expected_out = pr_read_test_file(
            "tests/pr/lorem_ipsum_output_merge.txt",
            input,
            None,
            Some(date.to_string()),
        );

        assert_eq!(stdout, expected_out);
    });
}

#[test]
fn test_pr_page_skip() {
    let input = "tests/pr/numbers.txt";
    let output = pr_read_test_file(
        "tests/pr/numbers_output_9_cols_page15.txt",
        input,
        None,
        None,
    );
    pr_test(&["-9", "+15", &input], "", &output);
}

#[test]
fn test_pr_header_replacement() {
    let header = "custom";
    let input = "tests/pr/lorem_ipsum.txt";
    let output = pr_read_test_file(
        "tests/pr/lorem_ipsum_output_page_1.txt",
        input,
        Some(header),
        None,
    );
    pr_test(&["+1:1", "-h", header, &input], "", &output);
}

#[test]
fn test_pr_limit_lines() {
    let input = "tests/pr/numbers.txt";
    let output = pr_read_test_file("tests/pr/numbers_output_l20.txt", input, None, None);
    pr_test(&["+1:1", "-l20", &input], "", &output);
}

#[test]
fn test_pr_limit_lines_trim() {
    // Lines <= 10 behave like -t is used
    let input = "tests/pr/numbers.txt";
    let output = pr_read_test_file("tests/pr/numbers_output_l10.txt", input, None, None);
    pr_test(&["+1:1", "-l10", &input], "", &output);
}

#[test]
fn test_pr_omit_header() {
    let input = "tests/pr/numbers.txt";
    let output = pr_read_test_file("tests/pr/numbers_output_omit_header.txt", input, None, None);
    pr_test(&["+1:1", "-l20", "-t", &input], "", &output);
}

#[test]
fn test_pr_offset() {
    let input = "tests/pr/numbers.txt";
    let output = pr_read_test_file("tests/pr/numbers_output_offset.txt", input, None, None);
    pr_test(&["+1:1", "-o7", &input], "", &output);
}

#[test]
fn test_pr_width() {
    let input = "tests/pr/long_line.txt";
    let output = pr_read_test_file("tests/pr/long_line_output_w72.txt", input, None, None);
    pr_test(&["-2", "-t", "-w72", &input], "", &output);

    let output = pr_read_test_file("tests/pr/long_line_output_w200.txt", input, None, None);
    pr_test(&["-2", "-t", "-w200", &input], "", &output);

    // -s without -w causes the width to be 512
    let output = pr_read_test_file("tests/pr/long_line_output_s.txt", input, None, None);
    pr_test(&["-2", "-t", "-s", &input], "", &output);
}

#[test]
fn test_pr_number_line() {
    let input = "tests/pr/lorem_ipsum.txt";
    let output = pr_read_test_file(
        "tests/pr/lorem_ipsum_output_number_line.txt",
        input,
        None,
        None,
    );
    pr_test(&["-9", "-n3", &input], "", &output);
}

#[test]
fn test_pr_expand_and_replace() {
    let input = "tests/pr/spaces_and_tabs.txt";
    let output = pr_read_test_file(
        "tests/pr/spaces_and_tabs_expand_and_replace.txt",
        input,
        None,
        None,
    );
    pr_test(&["-i?3", "-e", "-t", &input], "", &output);
}
