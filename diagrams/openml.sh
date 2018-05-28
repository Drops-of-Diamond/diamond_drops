if [[ "$OSTYPE" == "linux-gnu" ]]; then
    if lsb_release -a | grep -q 'Manjaro'; then
        echo "Manjaro Linux distro detected via lsb_release -a, installing yaourt and python-pygraphviz dependencies if needed for MML, otherwise this will skip install if not needed";
        sudo pacman -S --noconfirm --needed yaourt;
        yaourt -S --noconfirm --needed python-pygraphviz;
    elif lsb_release -a | grep -q 'Ubuntu' || 'Debian'; then
        sudo apt-add-repository ppa:dperry/ppa-graphviz-test;
        sudo apt-get update;
        sudo apt-get autoremove graphviz;
        if sudo apt-get install graphviz | grep -q 'graphviz : Depends: libgraphviz4 (>= 2.18) but it is not going to be installed'; then
            sudo apt-get remove libcdt4;
            sudo apt-get remove libpathplan4;
            sudo apt-get install graphviz;
        fi
        sudo apt-get install graphviz;
    else
        echo "Script detected a Linux distro that the script has not set up for installing graphviz, please install manually and try again or make a pull request"
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    open ./diagrams/ml.svg;
`#elif [[ "$OSTYPE" == "cygwin" ]]; then`
        `# POSIX compatibility layer and Linux environment emulation for Windows`
`#elif [[ "$OSTYPE" == "msys" ]]; then`
        `# Lightweight shell and GNU utilities compiled for Windows (part of MinGW)`
elif [[ "$OSTYPE" == "win32" ]]; then
        `# I'm not sure this can happen.`
    start ./diagrams/ml.svg;
`#elif [[ "$OSTYPE" == "freebsd"* ]]; then`
        # ...
#else
        # Unknown.
fi