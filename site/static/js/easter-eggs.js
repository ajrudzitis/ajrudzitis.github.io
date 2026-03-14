// Easter eggs - whimsical surprises
(function() {
    'use strict';

    let quotes = [];
    let quotesLoaded = false;

    // Load quotes on first interaction
    async function loadQuotes() {
        if (quotesLoaded) return;
        try {
            const response = await fetch('/quotes/quotes.txt');
            const text = await response.text();
            quotes = text.split('\n').filter(q => q.trim().length > 0);
            quotesLoaded = true;
        } catch (e) {
            console.log('Could not load quotes');
        }
    }

    // Konami Code: up up down down left right left right b a
    const konamiCode = ['ArrowUp', 'ArrowUp', 'ArrowDown', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'ArrowLeft', 'ArrowRight', 'b', 'a'];
    let konamiIndex = 0;

    document.addEventListener('keydown', async function(e) {
        if (e.key === konamiCode[konamiIndex]) {
            konamiIndex++;
            if (konamiIndex === konamiCode.length) {
                konamiIndex = 0;
                await loadQuotes();
                if (quotes.length > 0) {
                    showFloatingQuote();
                }
            }
        } else {
            konamiIndex = 0;
        }
    });

    // Creative quote display - floating bubble that drifts across screen
    function showFloatingQuote() {
        const quote = quotes[Math.floor(Math.random() * quotes.length)];

        const bubble = document.createElement('div');
        bubble.className = 'quote-bubble';
        bubble.textContent = quote;

        // Random starting position on left edge
        const startY = Math.random() * 60 + 20; // 20-80% from top

        bubble.style.cssText = `
            position: fixed;
            left: -400px;
            top: ${startY}%;
            max-width: 350px;
            padding: 20px 25px;
            background: rgba(255, 255, 255, 0.95);
            border: 2px solid #333;
            border-radius: 20px;
            font-family: Georgia, serif;
            font-size: 18px;
            font-style: italic;
            line-height: 1.5;
            color: #333;
            box-shadow: 0 10px 40px rgba(0,0,0,0.2);
            z-index: 10001;
            transform: rotate(${Math.random() * 6 - 3}deg);
            transition: none;
            cursor: pointer;
        `;

        document.body.appendChild(bubble);

        // Animate across screen
        let x = -400;
        const y = parseFloat(bubble.style.top);
        const speed = 1.5 + Math.random();
        const wobble = Math.random() * 0.5;
        let time = 0;

        function animate() {
            time += 0.02;
            x += speed;
            const yOffset = Math.sin(time * 2) * 15 * wobble;
            bubble.style.left = x + 'px';
            bubble.style.top = `calc(${y}% + ${yOffset}px)`;

            if (x < window.innerWidth + 100) {
                requestAnimationFrame(animate);
            } else {
                bubble.remove();
            }
        }

        requestAnimationFrame(animate);

        // Click to dismiss
        bubble.addEventListener('click', () => bubble.remove());
    }

    // Footer click counter - 7 clicks reveals a quote
    let footerClicks = 0;
    let footerClickTimeout;

    document.addEventListener('DOMContentLoaded', function() {
        const footer = document.querySelector('footer');
        if (footer) {
            footer.addEventListener('click', async function(e) {
                if (e.target.tagName === 'A') return; // Don't interfere with links

                footerClicks++;
                clearTimeout(footerClickTimeout);

                if (footerClicks >= 7) {
                    footerClicks = 0;
                    await loadQuotes();
                    if (quotes.length > 0) {
                        showFooterQuote();
                    }
                } else {
                    footerClickTimeout = setTimeout(() => {
                        footerClicks = 0;
                    }, 2000);
                }
            });
        }
    });

    function showFooterQuote() {
        const quote = quotes[Math.floor(Math.random() * quotes.length)];
        const footer = document.querySelector('footer');

        // Remove existing quote if any
        const existing = document.getElementById('footer-quote');
        if (existing) existing.remove();

        const quoteEl = document.createElement('p');
        quoteEl.id = 'footer-quote';
        quoteEl.textContent = `"${quote}"`;
        quoteEl.style.cssText = `
            font-style: italic;
            font-size: 0.9rem;
            margin-top: 1rem;
            opacity: 0;
            transition: opacity 0.5s ease;
        `;

        footer.appendChild(quoteEl);

        // Fade in
        requestAnimationFrame(() => {
            quoteEl.style.opacity = '0.8';
        });

        // Fade out after 8 seconds
        setTimeout(() => {
            quoteEl.style.opacity = '0';
            setTimeout(() => quoteEl.remove(), 500);
        }, 8000);
    }

    // Console greeting with a contemplative touch
    console.log('%c\u2728 Hello, curious one.', 'font-size: 16px; font-weight: bold;');
    console.log('%cAm I a programmer who takes walks, or a walker who occasionally debugs?', 'font-style: italic; color: #666;');
    console.log('%cTry the Konami code if you need a thought.', 'color: #999; font-size: 11px;');

    // Idle meditation prompt - gentle nudge after 2 min of inactivity
    let idleTimer;
    let idlePromptShown = false;

    function resetIdleTimer() {
        clearTimeout(idleTimer);
        if (idlePromptShown) return; // Only show once per page load

        idleTimer = setTimeout(showIdlePrompt, 120000); // 2 minutes
    }

    function showIdlePrompt() {
        if (idlePromptShown) return;
        idlePromptShown = true;

        const prompt = document.createElement('div');
        prompt.id = 'idle-prompt';
        prompt.innerHTML = `
            <div style="font-size: 24px; margin-bottom: 8px;">~</div>
            <div>Take a breath.</div>
        `;
        prompt.style.cssText = `
            position: fixed;
            bottom: 30px;
            left: 50%;
            transform: translateX(-50%);
            padding: 15px 30px;
            background: rgba(255, 255, 255, 0.95);
            border: 1px solid #ddd;
            border-radius: 8px;
            font-family: Georgia, serif;
            font-size: 14px;
            color: #666;
            text-align: center;
            box-shadow: 0 4px 20px rgba(0,0,0,0.1);
            z-index: 10000;
            opacity: 0;
            transition: opacity 0.8s ease;
            cursor: pointer;
        `;

        document.body.appendChild(prompt);

        requestAnimationFrame(() => {
            prompt.style.opacity = '1';
        });

        // Click to dismiss
        prompt.addEventListener('click', () => {
            prompt.style.opacity = '0';
            setTimeout(() => prompt.remove(), 800);
        });

        // Auto dismiss after 10 seconds
        setTimeout(() => {
            prompt.style.opacity = '0';
            setTimeout(() => prompt.remove(), 800);
        }, 10000);
    }

    // Track activity
    ['mousemove', 'keydown', 'scroll', 'click'].forEach(event => {
        document.addEventListener(event, resetIdleTimer, { passive: true });
    });

    // Start idle timer
    resetIdleTimer();

})();
