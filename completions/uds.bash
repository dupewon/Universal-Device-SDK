# UDS CLI bash completion
# Source with: source completions/uds.bash

_uds_completions() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local prev="${COMP_WORDS[COMP_CWORD-1]}"

    local commands="init devices inspect doctor logs monitor flash update benchmark plugins rpc fs build firmware generate docs version"

    local global_flags="--config --log-level --output --transport --device --help --version"

    case "$prev" in
        uds)
            COMPREPLY=($(compgen -W "$commands $global_flags" -- "$cur"))
            return 0
            ;;
        --config|--log-level|--output|--transport|--device)
            return 0
            ;;
        devices)
            COMPREPLY=($(compgen -W "--scan --watch" -- "$cur"))
            return 0
            ;;
        flash)
            COMPREPLY=($(compgen -f -- "$cur"))
            return 0
            ;;
        rpc)
            COMPREPLY=($(compgen -W "--device-id" -- "$cur"))
            return 0
            ;;
        plugins)
            COMPREPLY=($(compgen -W "list install remove" -- "$cur"))
            return 0
            ;;
        fs)
            COMPREPLY=($(compgen -W "ls cat cp mv rm mkdir" -- "$cur"))
            return 0
            ;;
        firmware)
            COMPREPLY=($(compgen -W "list verify sign" -- "$cur"))
            return 0
            ;;
        generate)
            COMPREPLY=($(compgen -W "--lang --output" -- "$cur"))
            return 0
            ;;
        *)
            COMPREPLY=()
            return 0
            ;;
    esac
}

complete -F _uds_completions uds
