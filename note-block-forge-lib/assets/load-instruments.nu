echo "Loading instrument assets..."

open $'($env.APPDATA)/.minecraft/assets/indexes/24.json'
    | get objects
    | transpose
    | rename name
    | where name starts-with "minecraft/sounds/note/"
    | flatten
    | each { mklink $'"instruments\($in.name | str replace "minecraft/sounds/note/" "")"' $'"($env.APPDATA)\.minecraft\assets\objects\($in.hash | str substring 0..2)\($in.hash)"' }
    | null
