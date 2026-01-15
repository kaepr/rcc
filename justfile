os := os()
arch := arch()

prefix := if os == "macos" { 
    if arch == "x86_64" { 
        "" 
    } else { 
        "arch -x86_64 " + env_var_or_default("SHELL", "sh") + " -c" 
    }
} else { 
    "" 
}

default:
    @just --list --unsorted

check-arch:
    {{ if prefix == "" { "" } else { prefix + " '" } }} uname -m {{ if prefix == "" { "" } else { "'" } }}

clean:
    @echo "Cleaning build artifacts..."
    rm -rf target


