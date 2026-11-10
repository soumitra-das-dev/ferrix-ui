# Matrix Run Summary: 20261110_120019

| Case ID | Template | Options | Status | Duration | Steps Passed | Failure Details |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| neg-collision-refuse | hello-counter | --collision-test | 🟢 PASSED | 1s | 2/2 | Directory collision correctly guarded |
| neg-unknown-template | non-existent-template | --yes | 🟢 PASSED | 1s | 1/1 | Unknown template cleanly rejected |
| tmpl-hello-counter-default | hello-counter | --yes | 🟢 PASSED | 98s | 9/9 | Clean pass with zero errors |
| tmpl-hello-counter-all-features | hello-counter | --router --navigation --storage --sqlite --tailwind --i18n --network --ci github --git | 🟢 PASSED | 85s | 9/9 | Clean pass with zero errors |
| tmpl-hello-counter-minimal | hello-counter | --no-router --no-tailwind --no-git | 🟢 PASSED | 50s | 9/9 | Clean pass with zero errors |
| tmpl-blank-default | blank | --yes | 🟢 PASSED | 75s | 9/9 | Clean pass with zero errors |
| tmpl-blank-all-features | blank | --router --navigation --storage --sqlite --tailwind --i18n --network | 🟢 PASSED | 81s | 9/9 | Clean pass with zero errors |
| tmpl-tabs-default | tabs | --yes | ❌ FAILED | 10s | 3/9 | Headless tests failed |
