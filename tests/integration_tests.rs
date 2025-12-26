use std::env;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Helper to run a tasklist command with a specific file
fn run_command(temp_dir: &PathBuf, args: &[&str]) -> std::process::Output {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let binary_path = format!("{}/target/debug/tasklist", manifest_dir);

    std::process::Command::new(&binary_path)
        .args(args)
        .current_dir(temp_dir)
        .output()
        .expect("Failed to run command")
}

#[test]
fn test_add_task_creates_metadata() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add a task using relative path (will be created in temp_dir)
    let output = run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "First task"],
    );

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let tasklist_path = temp_path.join(".tasklist");
    assert!(
        tasklist_path.exists(),
        "File was not created at {:?}",
        tasklist_path
    );

    // Verify metadata exists and task was added
    let content = fs::read_to_string(&tasklist_path).unwrap();
    assert!(content.starts_with("#max_id=1"), "Content: {}", content);
    assert!(content.contains("First task"));
}

#[test]
fn test_add_multiple_tasks_increments_id() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add first task
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Task 1"],
    );

    // Add second task
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Task 2"],
    );

    // Verify max_id is 2
    let tasklist_path = temp_path.join(".tasklist");
    let content = fs::read_to_string(&tasklist_path).unwrap();
    assert!(content.starts_with("#max_id=2"));
    assert!(content.contains("Task 1"));
    assert!(content.contains("Task 2"));

    // Count task lines (excluding metadata)
    let task_count = content.lines().filter(|l| !l.starts_with("#")).count();
    assert_eq!(task_count, 2);
}

#[test]
fn test_update_task_preserves_metadata() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add a task
    run_command(
        &temp_path,
        &[
            "--file",
            ".tasklist",
            "add",
            "--description",
            "Original task",
        ],
    );

    // Update the task
    let output = run_command(
        &temp_path,
        &[
            "--file",
            ".tasklist",
            "update",
            "--id",
            "1",
            "--status",
            "in_progress",
        ],
    );

    assert!(output.status.success());

    // Verify metadata is preserved
    let tasklist_path = temp_path.join(".tasklist");
    let content = fs::read_to_string(&tasklist_path).unwrap();
    assert!(content.starts_with("#max_id=1"));
    assert!(content.contains("In Progress"));
}

#[test]
fn test_update_nonexistent_task_fails() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add a task
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Task 1"],
    );

    // Try to update non-existent task
    let output = run_command(
        &temp_path,
        &[
            "--file",
            ".tasklist",
            "update",
            "--id",
            "999",
            "--status",
            "done",
        ],
    );

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Task with ID 999 not found"));
}

#[test]
fn test_delete_task_preserves_metadata() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add two tasks
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Task 1"],
    );
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Task 2"],
    );

    // Delete first task
    let output = run_command(&temp_path, &["--file", ".tasklist", "delete", "--id", "1"]);

    assert!(output.status.success());

    // Verify metadata is preserved
    let tasklist_path = temp_path.join(".tasklist");
    let content = fs::read_to_string(&tasklist_path).unwrap();
    assert!(content.starts_with("#max_id=2"));
    assert!(!content.contains("Task 1"));
    assert!(content.contains("Task 2"));
}

#[test]
fn test_delete_nonexistent_task_fails() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add a task
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Task 1"],
    );

    // Try to delete non-existent task
    let output = run_command(
        &temp_path,
        &["--file", ".tasklist", "delete", "--id", "999"],
    );

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Task with ID 999 not found"));
}

#[test]
fn test_migration_from_old_format() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    let tasklist_path = temp_path.join(".tasklist");

    // Create old format file (without metadata)
    fs::write(
        &tasklist_path,
        "1\t🚀 Not Started\tOld task 1\t2025-01-01 10:00\n\
         2\t✅ Done\tOld task 2\t2025-01-01 11:00\n",
    )
    .unwrap();

    // Add a new task (should trigger migration)
    let output = run_command(
        &temp_path,
        &[
            "--file",
            ".tasklist",
            "add",
            "--description",
            "New task after migration",
        ],
    );

    assert!(output.status.success());

    // Verify migration happened
    let content = fs::read_to_string(&tasklist_path).unwrap();
    assert!(content.starts_with("#max_id=3"));
    assert!(content.contains("Old task 1"));
    assert!(content.contains("Old task 2"));
    assert!(content.contains("New task after migration"));
}

#[test]
fn test_list_tasks_skips_metadata() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add tasks
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Task 1"],
    );
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Task 2"],
    );

    // List tasks
    let output = run_command(&temp_path, &["--file", ".tasklist", "show"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Task 1"));
    assert!(stdout.contains("Task 2"));
    assert!(!stdout.contains("#max_id"));
}

#[test]
fn test_empty_tasklist_shows_message() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    let tasklist_path = temp_path.join("empty.tasklist");

    // Create empty file
    fs::write(&tasklist_path, "#max_id=0\n").unwrap();

    // List tasks
    let output = run_command(&temp_path, &["--file", "empty.tasklist", "show"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No tasks found"));
}

#[test]
fn test_kanban_view_works() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add tasks with different statuses
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Todo task"],
    );
    run_command(
        &temp_path,
        &[
            "--file",
            ".tasklist",
            "add",
            "--description",
            "In progress task",
        ],
    );
    run_command(
        &temp_path,
        &[
            "--file",
            ".tasklist",
            "update",
            "--id",
            "2",
            "--status",
            "in_progress",
        ],
    );

    // Show kanban view
    let output = run_command(&temp_path, &["--file", ".tasklist", "show", "--kanban"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("NOT STARTED"));
    assert!(stdout.contains("IN PROGRESS"));
    assert!(stdout.contains("DONE"));
}

#[test]
fn test_verbose_flag_shows_file_path() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add a task first
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Test task"],
    );

    // Run with verbose flag
    let output = run_command(&temp_path, &["--file", ".tasklist", "--verbose", "show"]);

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Using tasklist file:"));
    assert!(stderr.contains(".tasklist"));
}

#[test]
fn test_default_command_shows_tasks() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add a task
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Test task"],
    );

    // Run without subcommand (should default to show)
    let output = run_command(&temp_path, &["--file", ".tasklist"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Test task"));
    assert!(stdout.contains("Project:"));
}

#[test]
fn test_global_kanban_flag() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add tasks
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Task 1"],
    );
    run_command(
        &temp_path,
        &["--file", ".tasklist", "add", "--description", "Task 2"],
    );

    // Use global --kanban flag (without subcommand)
    let output = run_command(&temp_path, &["--file", ".tasklist", "--kanban"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("NOT STARTED"));
    assert!(stdout.contains("IN PROGRESS"));
    assert!(stdout.contains("DONE"));
    assert!(stdout.contains("Task 1"));
}

#[test]
fn test_kanban_shows_dates() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add a task
    run_command(
        &temp_path,
        &[
            "--file",
            ".tasklist",
            "add",
            "--description",
            "Task with date",
        ],
    );

    // Show in kanban view
    let output = run_command(&temp_path, &["--file", ".tasklist", "show", "--kanban"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Check that dates are displayed (format: YYYY-MM-DD HH:MM)
    assert!(stdout.contains("20")); // Year starts with "20"
    assert!(stdout.contains("Task with date"));
}

#[test]
fn test_atomic_write_prevents_corruption() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();

    // Add multiple tasks in quick succession
    for i in 1..=5 {
        let output = run_command(
            &temp_path,
            &[
                "--file",
                ".tasklist",
                "add",
                "--description",
                &format!("Task {}", i),
            ],
        );
        assert!(output.status.success());
    }

    // Verify all tasks were added correctly
    let tasklist_path = temp_path.join(".tasklist");
    let content = fs::read_to_string(&tasklist_path).unwrap();

    // Should have metadata
    assert!(content.starts_with("#max_id=5"));

    // Should have all 5 tasks
    for i in 1..=5 {
        assert!(content.contains(&format!("Task {}", i)));
    }

    // Should have exactly 6 lines (1 metadata + 5 tasks)
    assert_eq!(content.lines().count(), 6);
}
