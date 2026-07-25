# UDS CLI zsh completion
# Source with: source completions/uds.zsh

#compdef uds

_uds_commands() {
    local -a commands
    commands=(
        'init:Initialize a new UDS project'
        'devices:List and discover connected devices'
        'inspect:Show detailed device information'
        'doctor:Run system diagnostics'
        'logs:Tail device logs'
        'monitor:Real-time device monitoring'
        'flash:Flash firmware image to device'
        'update:OTA firmware update'
        'benchmark:Run performance benchmarks'
        'plugins:Manage UDS plugins'
        'rpc:Invoke an RPC method'
        'fs:Filesystem operations on device'
        'build:Build firmware from source'
        'firmware:Manage firmware images'
        'generate:Generate code from IDL definitions'
        'docs:Open documentation'
        'version:Print version information'
    )
    _describe 'uds command' commands
}

_uds() {
    local context state state_descr line
    typeset -A opt_args

    _arguments -C \
        '--config[Config file path]' \
        '--log-level[Log level]' \
        '--output[Output format]' \
        '--transport[Transport to use]' \
        '--device[Target device ID]' \
        '--help[Show help]' \
        '--version[Show version]' \
        '1: :->command' \
        '*: :->args'

    case $state in
        command) _uds_commands ;;
        args) case $line[1] in
            plugins) _arguments '2: :(list install remove)' ;;
            fs) _arguments '2: :(ls cat cp mv rm mkdir)' ;;
            firmware) _arguments '2: :(list verify sign)' ;;
            flash) _files ;;
        esac ;;
    esac
}

_uds "$@"
