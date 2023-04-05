class ExecutionLock {
    isLocked = false;
    lockedPromise = null;
    autoUnlockAfterMilliseconds = 10000;

    lock(autoUnlockCallback) {
        this.locked = true;

        this.lockedPromise = new Promise((resolve, reject) => {
            let interval, timeout;

            // auto-unlock after a certain amount of time
            timeout = setTimeout(async () => {
                clearInterval(interval);
                await autoUnlockCallback();
                this.unlock();
                resolve("autounlock");
            }, this.autoUnlockAfterMilliseconds);

            // Check every 10ms if the lock is still locked
            interval = setInterval(() => {
                if (!this.isLocked) {
                    clearTimeout(timeout);
                    clearInterval(interval);
                    resolve("unlock");
                }
            }, 10);
        });
    }

    unlock() {
        this.isLocked = false;
    }

    async waitForUnlock() {
        if (!this.isLocked) {
            return;
        }

        await this.lockedPromise;
    }
}
