// Terminal Easter Egg - Blue Screen Terminal
// Triggered by Ctrl+Shift+X
// Future: Can connect to SSH backend for full terminal experience

(function() {
    'use strict';

    let terminal = null;
    let overlay = null;
    let isActive = false;
    let quotes = [];
    let quotesLoaded = false;
    let currentInput = '';

    const PROMPT = '> ';

    // Load quotes from the shared quotes file
    async function loadQuotes() {
        if (quotesLoaded) return;
        try {
            const response = await fetch('/quotes/quotes.txt');
            const text = await response.text();
            quotes = text.split('\n').filter(q => q.trim().length > 0);
            quotesLoaded = true;
        } catch (e) {
            console.log('Terminal egg: Could not load quotes');
            quotes = ['The universe is under no obligation to make sense to you. - Neil deGrasse Tyson'];
            quotesLoaded = true;
        }
    }

    function getRandomQuote() {
        if (quotes.length === 0) return 'No quotes available.';
        return quotes[Math.floor(Math.random() * quotes.length)];
    }

    // Create the terminal overlay structure
    function createOverlay() {
        if (overlay) return;

        overlay = document.createElement('div');
        overlay.id = 'terminal-egg-overlay';
        overlay.innerHTML = `
            <div id="terminal-egg-container"></div>
            <div id="terminal-egg-help">Press ESC or type 'exit' to return | Ctrl+Shift+X to toggle</div>
        `;
        document.body.appendChild(overlay);
    }

    // Initialize xterm.js terminal
    function initTerminal() {
        if (terminal) return;

        const container = document.getElementById('terminal-egg-container');

        terminal = new Terminal({
            cursorBlink: true,
            cursorStyle: 'block',
            fontFamily: '"Courier New", monospace',
            fontSize: 16,
            lineHeight: 1.2,
            theme: {
                background: '#0000aa',
                foreground: '#ffffff',
                cursor: '#ffffff',
                cursorAccent: '#0000aa',
                selectionBackground: '#ffffff',
                selectionForeground: '#0000aa'
            },
            allowTransparency: false,
            scrollback: 1000
        });

        terminal.open(container);

        // Handle terminal resize
        function fitTerminal() {
            const containerEl = document.getElementById('terminal-egg-container');
            if (!containerEl || !terminal) return;

            const dims = calculateTerminalSize(containerEl);
            terminal.resize(dims.cols, dims.rows);
        }

        function calculateTerminalSize(container) {
            // Try to get actual character dimensions from xterm
            let charWidth = 9.6;
            let charHeight = 19.2;

            try {
                const core = terminal._core;
                if (core && core._renderService && core._renderService.dimensions) {
                    charWidth = core._renderService.dimensions.css.cell.width || charWidth;
                    charHeight = core._renderService.dimensions.css.cell.height || charHeight;
                }
            } catch (e) {
                // Fall back to estimates
            }

            const padding = 40; // 20px padding on each side
            const helpTextHeight = 60; // Space for help text at bottom
            const safetyMargin = 2; // Extra rows buffer
            const width = container.clientWidth - padding;
            const height = container.clientHeight - padding - helpTextHeight;

            return {
                cols: Math.floor(width / charWidth),
                rows: Math.floor(height / charHeight) - safetyMargin
            };
        }

        // Initial fit
        setTimeout(fitTerminal, 100);
        window.addEventListener('resize', fitTerminal);

        // Handle keyboard input
        terminal.onData(data => {
            handleInput(data);
        });

        // Show welcome message
        showWelcome();
    }

    function showWelcome() {
        terminal.writeln('');
        terminal.writeln('\x1b[1m  ╔══════════════════════════════════════════════════════════╗\x1b[0m');
        terminal.writeln('\x1b[1m  ║                                                          ║\x1b[0m');
        terminal.writeln('\x1b[1m  ║               WELCOME TO THE TERMINAL                    ║\x1b[0m');
        terminal.writeln('\x1b[1m  ║                                                          ║\x1b[0m');
        terminal.writeln('\x1b[1m  ║     Press ENTER for wisdom. Type "exit" to leave.        ║\x1b[0m');
        terminal.writeln('\x1b[1m  ║                                                          ║\x1b[0m');
        terminal.writeln('\x1b[1m  ╚══════════════════════════════════════════════════════════╝\x1b[0m');
        terminal.writeln('');
        terminal.write(PROMPT);
        terminal.scrollToBottom();
    }

    function handleInput(data) {
        // Handle special characters
        for (let i = 0; i < data.length; i++) {
            const char = data[i];
            const code = char.charCodeAt(0);

            if (code === 13) {
                // Enter key
                terminal.writeln('');
                processCommand(currentInput.trim());
                currentInput = '';
                terminal.write(PROMPT);
                terminal.scrollToBottom();
            } else if (code === 127 || code === 8) {
                // Backspace
                if (currentInput.length > 0) {
                    currentInput = currentInput.slice(0, -1);
                    terminal.write('\b \b');
                }
            } else if (code === 27) {
                // Escape - close terminal (handled separately via keydown)
            } else if (code >= 32 && code < 127) {
                // Printable characters
                currentInput += char;
                terminal.write(char);
            }
        }
    }

    function processCommand(input) {
        const cmd = input.toLowerCase();

        if (cmd === 'exit' || cmd === 'quit' || cmd === 'q') {
            hideTerminal();
            return;
        }

        if (cmd === 'help' || cmd === '?') {
            terminal.writeln('');
            terminal.writeln('  Commands:');
            terminal.writeln('    <enter>  - Display a random quote');
            terminal.writeln('    help     - Show this help message');
            terminal.writeln('    clear    - Clear the screen');
            terminal.writeln('    exit     - Return to the website');
            terminal.writeln('');
            return;
        }

        if (cmd === 'clear' || cmd === 'cls') {
            terminal.clear();
            terminal.write(PROMPT);
            return;
        }

        // Default: show a random quote
        const quote = getRandomQuote();
        terminal.writeln('');
        terminal.writeln('  ' + quote);
        terminal.writeln('');
    }

    async function showTerminal() {
        if (isActive) return;

        await loadQuotes();
        createOverlay();

        overlay.classList.add('active');
        isActive = true;

        // Initialize terminal on first show
        if (!terminal) {
            // Small delay to ensure overlay is visible and sized
            setTimeout(() => {
                initTerminal();
                terminal.focus();
            }, 50);
        } else {
            terminal.focus();
        }

        // Prevent body scroll while terminal is active
        document.body.style.overflow = 'hidden';
    }

    function hideTerminal() {
        if (!isActive) return;

        overlay.classList.remove('active');
        isActive = false;

        // Restore body scroll
        document.body.style.overflow = '';

        // Clear input state
        currentInput = '';
    }

    function toggleTerminal() {
        if (isActive) {
            hideTerminal();
        } else {
            showTerminal();
        }
    }

    // Keyboard shortcut: Ctrl+Shift+X
    document.addEventListener('keydown', function(e) {
        // Toggle terminal
        if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'X') {
            e.preventDefault();
            toggleTerminal();
            return;
        }

        // Escape to close
        if (e.key === 'Escape' && isActive) {
            e.preventDefault();
            hideTerminal();
            return;
        }
    });

    // Load xterm.js dynamically when needed
    let xtermLoaded = false;
    const originalShowTerminal = showTerminal;

    showTerminal = async function() {
        if (!xtermLoaded) {
            await loadXterm();
            xtermLoaded = true;
        }
        return originalShowTerminal();
    };

    function loadXterm() {
        return new Promise((resolve, reject) => {
            // Check if already loaded
            if (window.Terminal) {
                resolve();
                return;
            }

            // Load CSS
            const link = document.createElement('link');
            link.rel = 'stylesheet';
            link.href = 'https://cdn.jsdelivr.net/npm/xterm@5.3.0/css/xterm.css';
            document.head.appendChild(link);

            // Load JS
            const script = document.createElement('script');
            script.src = 'https://cdn.jsdelivr.net/npm/xterm@5.3.0/lib/xterm.min.js';
            script.onload = () => resolve();
            script.onerror = () => reject(new Error('Failed to load xterm.js'));
            document.head.appendChild(script);
        });
    }

    // Expose for potential programmatic use
    window.terminalEgg = {
        show: showTerminal,
        hide: hideTerminal,
        toggle: toggleTerminal,
        isActive: () => isActive
    };

    // Console hint
    console.log('%c\ud83d\udcbb Terminal mode available: Ctrl+Shift+X', 'color: #0000aa; font-weight: bold;');

})();
