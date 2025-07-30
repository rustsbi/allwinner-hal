# xfel to Rust rfel Refactoring To-Do List

## Phase 1: Foundational Work

- [ ] **Set up the basic Rust project structure.**
    - Define modules and file layout.
- [ ] **Define core data structures and constants.**
- [ ] **Implement command-line argument parsing.**
    - Define the structure for holding parsed arguments.
- [ ] **Re-implement simple, self-contained utility functions.**
    such as:
    - `filedump`
    - `hexdump`

## Phase 2: Core Logic and Library Migration (Could be LLM-Assisted)

- [ ] **See the "Core Logic Refactoring Workflow" section for detailed tasks.**

## Phase 3: Integration and Verification

- [ ] **Integrate all refactored modules.**
- [ ] **Write unit and integration tests.**
- [ ] **Perform end-to-end testing to ensure functionality matches the original C project.**

## Low Priority

- [ ] **Investigate and understand the `payload` directory.**


# LLM-Assisted Refactoring Workflow

## 1. Library and Dependency Replacement

* **Goal:** Replace C libraries and custom functions with modern, idiomatic Rust crates. An LLM can excel here due to its broad knowledge of libraries across different languages.
* **Workflow:**
    * [ ] **Identify Candidates:** Use an LLM to scan the C source code and identify all external library calls (e.g. crypto libraries for `sha256.h/.c`, `ecdsa256.h/.c`) and custom utility functions that have common equivalents.
    * [ ] **Suggest Alternatives:** Prompt the LLM to suggest popular and well-maintained Rust crates that provide equivalent functionality.
    * [ ] **Generate Wrapper Code:** Once a crate is chosen, ask the LLM to help generate the necessary wrapper code or usage examples to integrate the new crate into our project structure.

## 2. Automated Translation and Idiomatic Refactoring

* **Goal:** Translate the core C logic into safe, idiomatic Rust, improving upon the direct output of automated tools like `c2rust`.
* **Workflow:**
    * [ ] **Generate Baseline with `c2rust`:** Run the `c2rust` tool on a target C module to get an initial, albeit rough and unsafe, Rust translation.
    * [ ] **Semantic Refactoring with LLM:** Provide the LLM with both the **original C source code** (for semantic context) and the **`c2rust`-generated Rust code** (as a starting point).
    * [ ] **Prompt for Idiomatic Rewrite:** Instruct the LLM to refactor the `c2rust` output into safe, idiomatic Rust. This is the most critical step. (Notice that the context could not be too long, so maybe we need to break it into smaller parts.)
    * [ ] **Iterative Review:** This will be an iterative process. Start with small, pure functions and gradually move to more complex code. **Human oversight is critical** to validate the correctness and safety of the LLM's output.