alias d="docker"
alias k="kubectl"
alias c="clear"

if [ -f /etc/bash_completion ]; then
    . /etc/bash_completion
fi
if [ -f /root/.wasmedge/env ]; then
    . /root/.wasmedge/env
fi

source <(docker completion bash)
source <(kubectl completion bash)

complete -F _docker d
complete -F __start_kubectl k
