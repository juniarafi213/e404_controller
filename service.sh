#!/system/bin/sh
# E404 Controller - Boot Service Daemon

MODDIR="$(dirname "$0")"

# Wait until system is booted
while [ "$(getprop sys.boot_completed)" != "1" ]; do
    sleep 3
done

# Sleep additional 2s for thermal and power HAL initialization
sleep 2

# Set permissions
chmod 755 "$MODDIR/bin/e404-ctrl" 2>/dev/null
chmod 755 "$MODDIR/bin/e404-daemon" 2>/dev/null
chmod -R 755 "$MODDIR/webroot" 2>/dev/null

# Symlink CLI tool into KSU/Magisk bin so it is available in terminal PATH
if [ -d "/data/adb/ksu/bin" ]; then
    ln -sf "$MODDIR/bin/e404-ctrl" /data/adb/ksu/bin/e404-ctrl
    ln -sf "$MODDIR/bin/e404-daemon" /data/adb/ksu/bin/e404-daemon
elif [ -d "/data/adb/magisk" ]; then
    ln -sf "$MODDIR/bin/e404-ctrl" /data/adb/magisk/e404-ctrl
    ln -sf "$MODDIR/bin/e404-daemon" /data/adb/magisk/e404-daemon
fi

# Locate busybox
BUSYBOX="/data/adb/ksu/bin/busybox"
[ ! -x "$BUSYBOX" ] && BUSYBOX="/data/adb/magisk/busybox"
[ ! -x "$BUSYBOX" ] && BUSYBOX="$(which busybox 2>/dev/null)"

# Start WebUI local server on 127.0.0.1:8404
if [ -x "$BUSYBOX" ]; then
    # Kill existing instance on port 8404 if any
    pkill -f "8404" 2>/dev/null || true
    "$BUSYBOX" httpd -p 127.0.0.1:8404 -h "$MODDIR/webroot"
fi

# Apply saved profile and hardware tweaks
"$MODDIR/bin/e404-ctrl" apply >/dev/null 2>&1

# Start Auto-Game Daemon
"$MODDIR/bin/e404-daemon" start >/dev/null 2>&1 &

echo "[$(date '+%Y-%m-%d %H:%M:%S')] E404 Controller initialized successfully" > "$MODDIR/service.log"
exit 0
