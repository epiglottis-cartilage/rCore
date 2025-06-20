#![no_std]
#![no_main]

#[macro_use]
extern crate libr;

// not in SUCC_TESTS & FAIL_TESTS
// count_lines, infloop, user_shell, usertests

// item of TESTS : app_name(argv_0), argv_1, argv_2, argv_3, exit_code
static SUCC_TESTS: &[(&[&str], i32)] = &[
    (&["filetest_simple"], 0),
    (&["cat_filea"], 0),
    (&["exit"], 0),
    (&["fantastic_text", "0"], 0),
    (&["forktest"], 0),
    (&["forktest_simple"], 0),
    (&["forktest2"], 0),
    (&["forktree"], 0),
    (&["hello_world"], 0),
    (&["matrix"], 0),
    (&["huge_write"], 0),
    (&["sleep_simple"], 0),
    (&["sleep", "1000"], 0),
    (&["yield_test"], 0),
    (&["pipe_large_test"], 0),
    (&["pipetest"], 0),
];
static FAIL_TESTS: &[(&[&str], i32)] = &[
    (&["stack_overflow"], -11),
    (&["priv_csr"], -4),
    (&["priv_inst"], -4),
    (&["store_fault"], -11),
];
use libr::{exec, fork, waitpid};

fn run_tests(tests: &[(&[&str], i32)]) -> i32 {
    let mut pass_num = 0;
    for test in tests {
        println!("Usertests: Running {}", test.0[0]);
        let pid = fork();
        if pid == 0 {
            exec(test.0[0], &test.0);
            panic!("unreachable!");
        } else {
            let mut exit_code: i32 = Default::default();
            let wait_pid = waitpid(pid as usize, &mut exit_code);
            assert_eq!(pid, wait_pid);
            if exit_code == test.1 {
                // summary apps with  exit_code
                pass_num = pass_num + 1;
                println!(
                    "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
                    test.0[0], pid, exit_code
                );
            } else {
                // show error in red
                println!(
                    "\x1b[31mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
                    test.0[0], pid, exit_code
                );
            }
        }
    }
    pass_num
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let succ_num = run_tests(SUCC_TESTS);
    let err_num = run_tests(FAIL_TESTS);
    if succ_num == SUCC_TESTS.len() as i32 && err_num == FAIL_TESTS.len() as i32 {
        println!(
            "{} of sueecssed apps, {} of failed apps run correctly. \nUsertests passed!",
            SUCC_TESTS.len(),
            FAIL_TESTS.len()
        );
        return 0;
    } else {
        if succ_num != SUCC_TESTS.len() as i32 {
            println!(
                "all successed app_num is  {} , but only  passed {}",
                SUCC_TESTS.len(),
                succ_num
            );
        }
        if err_num != FAIL_TESTS.len() as i32 {
            println!(
                "all failed app_num is  {} , but only  passed {}",
                FAIL_TESTS.len(),
                err_num
            );
        }
    }
    println!(" Usertests failed!");
    return -1;
}
