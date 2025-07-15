begin
    set PATCH "Add .gitignore tailored to py project layout"
    set PROMPT "reorganizing py python project\nstart with gitignore"
    set FILE .gitignore

    echo "# Byte-compiled / optimized / DLL files
__pycache__/
*.py[cod]
*.pyo

# Virtual environment
.venv/
env/
venv/

# Editor settings and OS files
*.swp
*.swo
*~
.DS_Store

# Pyright, Pylance, etc.
.pyright/

# Compiled binaries
*.so
*.out
*.exe

# Logs and dumps
*.log
*.bak

# Project-specific
/bin/notation-cli

# Patch artifacts (if auto-generated)
patches/*.orig
patches/*.rej

# Tools cache or temp output
tools/*.tmp
tools/*.cache
tools/__pycache__/

# Ignore structure backup variants
structure.yml" > $FILE

    git add $FILE
    git commit -m "$PATCH

$PROMPT"
end
