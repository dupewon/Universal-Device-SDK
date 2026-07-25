# UDS CLI fish completion
# Source with: source completions/uds.fish

complete -c uds -f

# Global flags
complete -c uds -l config -d 'Config file path'
complete -c uds -l log-level -d 'Log level [error, warn, info, debug, trace]'
complete -c uds -l output -d 'Output format [human, json]'
complete -c uds -l transport -d 'Transport to use [serial, tcp, udp, ws, ble, usb, mock]'
complete -c uds -l device -d 'Target device ID'
complete -c uds -l help -d 'Show help'
complete -c uds -l version -d 'Show version'

# Subcommands
complete -c uds -n __fish_use_subcommand -a init -d 'Initialize a new UDS project'
complete -c uds -n __fish_use_subcommand -a devices -d 'List and discover connected devices'
complete -c uds -n __fish_use_subcommand -a inspect -d 'Show detailed device information'
complete -c uds -n __fish_use_subcommand -a doctor -d 'Run system diagnostics'
complete -c uds -n __fish_use_subcommand -a logs -d 'Tail device logs'
complete -c uds -n __fish_use_subcommand -a monitor -d 'Real-time device monitoring'
complete -c uds -n __fish_use_subcommand -a flash -d 'Flash firmware image to device'
complete -c uds -n __fish_use_subcommand -a update -d 'OTA firmware update'
complete -c uds -n __fish_use_subcommand -a benchmark -d 'Run performance benchmarks'
complete -c uds -n __fish_use_subcommand -a plugins -d 'Manage UDS plugins'
complete -c uds -n __fish_use_subcommand -a rpc -d 'Invoke an RPC method'
complete -c uds -n __fish_use_subcommand -a fs -d 'Filesystem operations'
complete -c uds -n __fish_use_subcommand -a build -d 'Build firmware from source'
complete -c uds -n __fish_use_subcommand -a firmware -d 'Manage firmware images'
complete -c uds -n __fish_use_subcommand -a generate -d 'Generate code from IDL'
complete -c uds -n __fish_use_subcommand -a docs -d 'Open documentation'
complete -c uds -n __fish_use_subcommand -a version -d 'Print version information'

# Subcommand flags and args
complete -c uds -n '__fish_seen_subcommand_from devices' -l scan -d 'Scan for new devices'
complete -c uds -n '__fish_seen_subcommand_from devices' -l watch -d 'Watch for device changes'
complete -c uds -n '__fish_seen_subcommand_from flash' -l verify -d 'Verify image after flashing'
complete -c uds -n '__fish_seen_subcommand_from flash' -l ota -d 'Perform OTA update'
complete -c uds -n '__fish_seen_subcommand_from flash' -l partition -d 'Target partition'
complete -c uds -n '__fish_seen_subcommand_from flash' -a '(__fish_complete_path)'
complete -c uds -n '__fish_seen_subcommand_from plugins' -a 'list install remove'
complete -c uds -n '__fish_seen_subcommand_from fs' -a 'ls cat cp mv rm mkdir'
complete -c uds -n '__fish_seen_subcommand_from firmware' -a 'list verify sign'
complete -c uds -n '__fish_seen_subcommand_from generate' -l lang -d 'Output language [rust, c, python, ts, go]'
complete -c uds -n '__fish_seen_subcommand_from generate' -l output -d 'Output directory'
