# Define the _oi function for Bash completion of the 'pooi' command
_oi() {
    local i cur prev opts cmds
    COMPREPLY=()            # Initialize the completion reply array
    cur="${COMP_WORDS[COMP_CWORD]}"    # Current word being completed
    prev="${COMP_WORDS[COMP_CWORD-1]}" # Previous word

    cmd=""   # Initialize command variable
    opts=""  # Initialize options variable

    # Loop through all command words to identify the 'pooi' command
    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            "$1")
                cmd="pooi"  # Set command to 'pooi' if the specified word matches
                ;;
            *)
                ;;
        esac
    done

    # Check the identified command and its options for auto-completion
    case "${cmd}" in
        pooi)
            # Define available options for 'pooi' command
            opts="-h -V -a -u -q -r -s -c -L -l -p --help --version --all --urls --quiet --raw --save --cache --clean --list --lang --pick <query>..."
            
            # Auto-complete options if current word starts with a dash or if it's the first word
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi

            # Auto-complete based on the previous word
            case "${prev}" in
                --lang)
                    COMPREPLY=($(compgen -f "${cur}"))  # File-based completion for language codes
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))  # File-based completion for language codes
                    return 0
                    ;;
                --pick)
                    # Complete based on a predefined list of selectors
                    COMPREPLY=($(compgen -W "basic1 basic2 clock conversions currency define holidays lists lyrics maths pronounce snippets1 snippets2 sports summary translate weather" -- "${cur}"))
                    return 0
                    ;;
                -p)
                    # Complete based on a predefined list of selectors
                    COMPREPLY=($(compgen -W "basic1 basic2 clock conversions currency define holidays lists lyrics maths pronounce snippets1 snippets2 sports summary translate weather" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()  # No completions for other cases
                    ;;
            esac
            # Auto-complete options for 'pooi' command
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

# Set the _oi function as the completion function for 'pooi' command
complete -F _oi -o bashdefault -o default pooi
