// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded "><a href="getting-started.html"><strong aria-hidden="true">1.</strong> Getting Started</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="installation.html"><strong aria-hidden="true">1.1.</strong> Installation</a></li><li class="chapter-item expanded "><a href="quick-start.html"><strong aria-hidden="true">1.2.</strong> Quick Start</a></li><li class="chapter-item expanded "><a href="first-chat.html"><strong aria-hidden="true">1.3.</strong> Your First Chat</a></li></ol></li><li class="chapter-item expanded "><a href="usage-integration.html"><strong aria-hidden="true">2.</strong> Usage &amp; Integration</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="chat-gipitty.html"><strong aria-hidden="true">2.1.</strong> Chat Gipitty Integration</a></li><li class="chapter-item expanded "><a href="python-integration.html"><strong aria-hidden="true">2.2.</strong> Python with OpenAI Library</a></li><li class="chapter-item expanded "><a href="curl-examples.html"><strong aria-hidden="true">2.3.</strong> Curl Examples</a></li><li class="chapter-item expanded "><a href="ollama.html"><strong aria-hidden="true">2.4.</strong> Ollama Integration</a></li></ol></li><li class="chapter-item expanded "><a href="api/overview.html"><strong aria-hidden="true">3.</strong> API Reference</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="api/chat-completions.html"><strong aria-hidden="true">3.1.</strong> Chat Completions Endpoint</a></li><li class="chapter-item expanded "><a href="api/search.html"><strong aria-hidden="true">3.2.</strong> Search &amp; Retrieval</a></li><li class="chapter-item expanded "><a href="api/data-management.html"><strong aria-hidden="true">3.3.</strong> Data Management</a></li><li class="chapter-item expanded "><a href="api/cli.html"><strong aria-hidden="true">3.4.</strong> Command Line Interface</a></li></ol></li><li class="chapter-item expanded "><a href="architecture/overview.html"><strong aria-hidden="true">4.</strong> Architecture &amp; Design</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="architecture/data-model.html"><strong aria-hidden="true">4.1.</strong> Data Model</a></li><li class="chapter-item expanded "><a href="architecture/context-enrichment.html"><strong aria-hidden="true">4.2.</strong> Context Enrichment</a></li><li class="chapter-item expanded "><a href="architecture/synapses.html"><strong aria-hidden="true">4.3.</strong> Conversation Threads (Synapses)</a></li></ol></li><li class="chapter-item expanded "><a href="features/providers.html"><strong aria-hidden="true">5.</strong> Features</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="features/token-management.html"><strong aria-hidden="true">5.1.</strong> Token Management</a></li><li class="chapter-item expanded "><a href="features/partitioning.html"><strong aria-hidden="true">5.2.</strong> Partitioning &amp; Organization</a></li><li class="chapter-item expanded "><a href="features/web-search.html"><strong aria-hidden="true">5.3.</strong> Web Search Integration</a></li><li class="chapter-item expanded "><a href="features/import-export.html"><strong aria-hidden="true">5.4.</strong> Import/Export</a></li></ol></li><li class="chapter-item expanded "><a href="deployment/local.html"><strong aria-hidden="true">6.</strong> Deployment &amp; Operations</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="deployment/production.html"><strong aria-hidden="true">6.1.</strong> Production Setup</a></li><li class="chapter-item expanded "><a href="deployment/environment.html"><strong aria-hidden="true">6.2.</strong> Environment Variables</a></li></ol></li><li class="chapter-item expanded "><a href="troubleshooting/common-issues.html"><strong aria-hidden="true">7.</strong> Help &amp; Support</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="troubleshooting/faq.html"><strong aria-hidden="true">7.1.</strong> FAQ</a></li><li class="chapter-item expanded "><a href="troubleshooting/debugging.html"><strong aria-hidden="true">7.2.</strong> Debugging</a></li></ol></li><li class="chapter-item expanded "><a href="development/contributing.html"><strong aria-hidden="true">8.</strong> Contributing</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="development/building.html"><strong aria-hidden="true">8.1.</strong> Building from Source</a></li><li class="chapter-item expanded "><a href="development/testing.html"><strong aria-hidden="true">8.2.</strong> Testing</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
