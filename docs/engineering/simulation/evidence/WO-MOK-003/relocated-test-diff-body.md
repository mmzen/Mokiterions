### `defaults_are_stable`

- from `src/cli.rs` to `tests/cli.rs`
- **Identical after dedenting.** No character of the test body changed.

### `options_work_in_any_order`

- from `src/cli.rs` to `tests/cli.rs`
- **Identical after dedenting.** No character of the test body changed.

### `both_policies_are_selectable_and_reference_is_the_default`

- from `src/cli.rs` to `tests/cli.rs`
- **Identical after dedenting.** No character of the test body changed.

### `duplicates_and_missing_values_are_rejected`

- from `src/cli.rs` to `tests/cli.rs`
- **Identical after dedenting.** No character of the test body changed.

### `density_is_accepted_in_the_specified_forms_and_rejected_otherwise`

- from `src/cli.rs` to `tests/cli.rs`
- **Identical after dedenting.** No character of the test body changed.

### `help_exits_successfully`

- from `src/main.rs` to `tests/process.rs`
- **Identical after dedenting.** No character of the test body changed.

### `invalid_configuration_exits_with_code_two`

- from `src/main.rs` to `tests/process.rs`
- **Identical after dedenting.** No character of the test body changed.

### `a_density_resolving_to_no_resources_exits_with_code_two_before_initialization`

- from `src/main.rs` to `tests/process.rs`
- **Identical after dedenting.** No character of the test body changed.

### `output_failure_exits_with_code_one`

- from `src/main.rs` to `tests/process.rs`
- **Identical after dedenting.** No character of the test body changed.

### `density_resolves_to_the_specified_resource_count`

- from `src/simulation.rs` to `tests/density.rs`
- **Identical after dedenting.** No character of the test body changed.

### `a_density_resolving_to_no_resources_is_rejected`

- from `src/simulation.rs` to `tests/density.rs`
- **Identical after dedenting.** No character of the test body changed.

### `tick_limit_terminates_with_one_summary`

- from `src/simulation.rs` to `tests/termination.rs`
- Changed lines:

```diff
--- src/simulation.rs (pre-change)
+++ tests/termination.rs
@@ -6,4 +6,4 @@
 
-    assert_eq!(summary.reason, TerminationReason::TickLimit);
-    assert_eq!(summary.ticks, 1);
+    assert_eq!(summary.reason(), TerminationReason::TickLimit);
+    assert_eq!(summary.ticks(), 1);
     assert_eq!(
```

### `a_long_configured_run_is_bounded_and_does_not_panic`

- from `src/simulation.rs` to `tests/termination.rs`
- Changed lines:

```diff
--- src/simulation.rs (pre-change)
+++ tests/termination.rs
@@ -5,4 +5,4 @@
 
-    assert!(summary.ticks <= 10_000);
-    assert_eq!(summary.survivors + summary.deaths, 12);
+    assert!(summary.ticks() <= 10_000);
+    assert_eq!(summary.survivors() + summary.deaths(), 12);
 }
```

### `a_long_run_is_bounded_under_either_source`

- from `src/simulation.rs` to `tests/termination.rs`
- Changed lines:

```diff
--- src/simulation.rs (pre-change)
+++ tests/termination.rs
@@ -13,4 +13,4 @@
 
-        assert!(summary.ticks <= 10_000);
-        assert_eq!(summary.survivors + summary.deaths, 12);
+        assert!(summary.ticks() <= 10_000);
+        assert_eq!(summary.survivors() + summary.deaths(), 12);
     }
```

### `the_reference_source_sustains_the_population_at_every_declared_density`

- from `src/simulation.rs` to `tests/viability.rs`
- Changed lines:

```diff
--- src/simulation.rs (pre-change)
+++ tests/viability.rs
@@ -13,3 +13,3 @@
             assert_eq!(
-                summary.reason,
+                summary.reason(),
                 TerminationReason::TickLimit,
@@ -18,5 +18,5 @@
             assert!(
-                summary.survivors >= floor,
+                summary.survivors() >= floor,
                 "seed {seed} at density {density}% left only {} survivors, below the stated floor of {floor}",
-                summary.survivors
+                summary.survivors()
             );
```
