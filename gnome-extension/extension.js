import GLib from 'gi://GLib';
import Gio from 'gi://Gio';
import Clutter from 'gi://Clutter';
import St from 'gi://St';
import * as Main from 'resource:///org/gnome/shell/ui/main.js';
import { Extension } from 'resource:///org/gnome/shell/extensions/extension.js';

const PARTICLE_COUNT = 300;
const DURATION_MS = 4000;
const COLORS = [
    '#FF3352', // Red
    '#FF8000', // Orange
    '#FFE600', // Yellow
    '#33CC33', // Green
    '#3399FF', // Blue
    '#9933CC', // Purple
    '#FF66B2', // Pink
    '#00E6E6', // Cyan
];

const DBUS_INTERFACE = `
<node>
  <interface name="com.github.ojii3.Confetti">
    <method name="Fire"/>
  </interface>
</node>
`;

class Particle {
    constructor(container, screenWidth, screenHeight) {
        this.screenWidth = screenWidth;
        this.screenHeight = screenHeight;

        const fromLeft = Math.random() < 0.5;
        this.x = fromLeft ? 0 : screenWidth;
        this.y = screenHeight * (0.3 + Math.random() * 0.4);

        const targetX = screenWidth / 2 + (Math.random() - 0.5) * 400;
        const targetY = screenHeight * (0.2 + Math.random() * 0.3);

        const speed = 800 + Math.random() * 700;
        const dx = targetX - this.x;
        const dy = targetY - this.y;
        const dist = Math.sqrt(dx * dx + dy * dy);

        this.vx = (dx / dist) * speed;
        this.vy = (dy / dist) * speed - (200 + Math.random() * 300);

        this.width = 8 + Math.random() * 8;
        this.height = 12 + Math.random() * 12;
        this.rotation = Math.random() * 360;
        this.rotationSpeed = (Math.random() - 0.5) * 600;

        const color = COLORS[Math.floor(Math.random() * COLORS.length)];

        this.actor = new St.Widget({
            style: `background-color: ${color}; border-radius: 2px;`,
            width: this.width,
            height: this.height,
            x: this.x - this.width / 2,
            y: this.y - this.height / 2,
            rotation_angle_z: this.rotation,
        });

        this.actor.set_pivot_point(0.5, 0.5);

        container.add_child(this.actor);
    }

    update(dt) {
        this.vy += 600 * dt;
        this.vx *= 0.99;
        this.vy *= 0.99;

        this.x += this.vx * dt;
        this.y += this.vy * dt;
        this.rotation += this.rotationSpeed * dt;

        this.actor.set_position(this.x - this.width / 2, this.y - this.height / 2);
        this.actor.set_rotation_angle(Clutter.RotateAxis.Z_AXIS, this.rotation);
    }

    setOpacity(opacity) {
        this.actor.set_opacity(Math.floor(opacity * 255));
    }

    destroy() {
        this.actor.destroy();
    }
}

class ConfettiEffect {
    constructor() {
        this.particles = [];
        this.container = null;
        this.timeoutId = null;
        this.startTime = null;
    }

    fire() {
        if (this.container) {
            this.cleanup();
        }

        const monitor = Main.layoutManager.primaryMonitor;
        const screenWidth = monitor.width;
        const screenHeight = monitor.height;

        this.container = new St.Widget({
            reactive: false,
            x: monitor.x,
            y: monitor.y,
            width: screenWidth,
            height: screenHeight,
        });

        Main.layoutManager.addTopChrome(this.container);

        this.particles = [];
        for (let i = 0; i < PARTICLE_COUNT; i++) {
            this.particles.push(new Particle(this.container, screenWidth, screenHeight));
        }

        this.startTime = GLib.get_monotonic_time() / 1000;
        this.lastTime = this.startTime;

        this.timeoutId = GLib.timeout_add(GLib.PRIORITY_DEFAULT, 16, () => {
            return this.update();
        });
    }

    update() {
        const now = GLib.get_monotonic_time() / 1000;
        const dt = (now - this.lastTime) / 1000;
        this.lastTime = now;

        const elapsed = now - this.startTime;

        if (elapsed > DURATION_MS) {
            this.cleanup();
            return GLib.SOURCE_REMOVE;
        }

        const fadeStart = DURATION_MS - 1000;
        const opacity = elapsed > fadeStart ? 1.0 - (elapsed - fadeStart) / 1000 : 1.0;

        for (const particle of this.particles) {
            particle.update(dt);
            particle.setOpacity(opacity);
        }

        return GLib.SOURCE_CONTINUE;
    }

    cleanup() {
        if (this.timeoutId) {
            GLib.source_remove(this.timeoutId);
            this.timeoutId = null;
        }

        for (const particle of this.particles) {
            particle.destroy();
        }
        this.particles = [];

        if (this.container) {
            Main.layoutManager.removeChrome(this.container);
            this.container.destroy();
            this.container = null;
        }
    }
}

export default class ConfettiExtension extends Extension {
    constructor(metadata) {
        super(metadata);
        this._dbus = null;
        this._effect = null;
        this._ownerId = null;
    }

    enable() {
        this._effect = new ConfettiEffect();

        this._dbus = Gio.DBusExportedObject.wrapJSObject(DBUS_INTERFACE, {
            Fire: () => {
                this._effect.fire();
            },
        });

        this._dbus.export(Gio.DBus.session, '/com/github/ojii3/Confetti');

        this._ownerId = Gio.DBus.session.own_name(
            'com.github.ojii3.Confetti',
            Gio.BusNameOwnerFlags.NONE,
            null,
            null
        );
    }

    disable() {
        if (this._effect) {
            this._effect.cleanup();
            this._effect = null;
        }

        if (this._ownerId) {
            Gio.DBus.session.unown_name(this._ownerId);
            this._ownerId = null;
        }

        if (this._dbus) {
            this._dbus.unexport();
            this._dbus = null;
        }
    }
}
