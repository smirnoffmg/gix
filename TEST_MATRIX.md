| ID   | Input Example                     | Expected Behavior             | Focus                    |
| ---- | --------------------------------- | ----------------------------- | ------------------------ |
| TC01 | `*.log\n*.log`                    | Remove duplicate              | Exact deduplication      |
| TC02 | `*.log\n*.log\n# comment`         | Keep one, preserve comment    | Comment handling         |
| TC03 | `*.log\n!debug.log`               | Keep both                     | Negation support         |
| TC04 | `/build\nbuild`                   | Keep both                     | Root vs relative         |
| TC05 | `\#notacomment`                   | Keep as-is                    | Escaped hash             |
| TC06 | `\!notnegation`                   | Keep as-is                    | Escaped negation         |
| TC07 | `*.log # inline`                  | Treat whole line              | Inline comment detection |
| TC08 | `debug/\n!debug/`                 | Keep both                     | Directory override       |
| TC09 | `*.log \n*.log`                   | Not identical – keep both     | Trailing space           |
| TC10 | `*.swp\n*.log\n*.swp`             | Dedup non-consecutive         | Line tracking            |
| TC11 | `node_modules/\n**/node_modules/` | Keep both                     | Wildcard semantics       |
| TC12 | `/tmp\n/tmp/`                     | Keep both                     | File vs directory        |
| TC13 | `# Logs\n*.log\n# Logs\n*.log`    | Optional warning              | Duplicate with comment   |
| TC14 | `build/\nbuild/`                  | Dedup                         | Directory repetition     |
| TC15 | `build/\nBUILD/`                  | Keep both                     | Case sensitivity         |
| TC16 | `# comment\n\n*.log\n`            | Ignore blank lines            | Layout preservation      |
| TC17 | `Данные/\n*.лог`                  | Keep both                     | Unicode entries          |
| TC18 | `**/*.log\n*.log`                 | Keep both                     | Wildcard range           |
| TC19 | `# 📝\n*.md\n*.md`                 | Dedup, preserve emoji comment | Emoji support            |
| TC20 | `foo\nfoo\n!foo`                  | Keep all                      | Pattern conflicts        |
