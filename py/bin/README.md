# \`bin/\` — Project CLI Scripts

This directory contains executable scripts used to run and manage the project from the command line.

## Conventions

- Scripts in \`bin/\` are **meant to be run directly**, e.g.:
  \`\`\`bash
  ./bin/notation-cli
  \`\`\`

- All scripts are:
  - Version-controlled
  - Have proper shebangs (e.g. \`#!/usr/bin/env fish\`)
  - Marked executable (\`chmod +x\`)

## Typical Usage

| Script          | Purpose                        |
|------------------|--------------------------------|
| \`notation-cli\`   | Launch the main CLI interface  |
| \`setup\` _(future)_ | Set up environment or dependencies |
| \`dev\` _(future)_   | Run development server/tools     |

## Notes

- Do **not** store compiled binaries or generated files here.
- This directory follows the **Rails-style convention**: all user-facing commands live here.
