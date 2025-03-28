echo "Loading minecraft assets..."

tar -xf $'($env.APPDATA)/.minecraft/versions/1.21.5/1.21.5.jar' assets/minecraft/textures/item assets/minecraft/textures/block

mkdir minecraft
mv assets/minecraft/textures/* ./minecraft
rm --permanent --recursive assets
