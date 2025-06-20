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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item affix "><li class="part-title">Quick Start</li><li class="chapter-item "><a href="installation.html"><strong aria-hidden="true">1.</strong> Installation</a></li><li class="chapter-item "><a href="quick-start.html"><strong aria-hidden="true">2.</strong> Getting Started</a></li><li class="chapter-item "><a href="first-chat.html"><strong aria-hidden="true">3.</strong> Your First Chat</a></li><li class="chapter-item affix "><li class="part-title">Usage Examples</li><li class="chapter-item "><a href="chat-gipitty.html"><strong aria-hidden="true">4.</strong> Chat Gipitty Integration</a></li><li class="chapter-item "><a href="python-integration.html"><strong aria-hidden="true">5.</strong> Python with OpenAI Library</a></li><li class="chapter-item "><a href="curl-examples.html"><strong aria-hidden="true">6.</strong> Curl Examples</a></li><li class="chapter-item "><a href="ollama.html"><strong aria-hidden="true">7.</strong> Ollama Integration</a></li><li class="chapter-item affix "><li class="part-title">API Reference</li><li class="chapter-item "><a href="api/overview.html"><strong aria-hidden="true">8.</strong> API Overview</a></li><li class="chapter-item "><a href="api/chat-completions.html"><strong aria-hidden="true">9.</strong> Chat Completions Endpoint</a></li><li class="chapter-item "><a href="api/search.html"><strong aria-hidden="true">10.</strong> Search &amp; Retrieval</a></li><li class="chapter-item "><a href="api/data-management.html"><strong aria-hidden="true">11.</strong> Data Management</a></li><li class="chapter-item "><a href="api/cli.html"><strong aria-hidden="true">12.</strong> Command Line Interface</a></li><li class="chapter-item affix "><li class="part-title">Architecture &amp; Design</li><li class="chapter-item "><a href="architecture/overview.html"><strong aria-hidden="true">13.</strong> System Architecture</a></li><li class="chapter-item "><a href="architecture/data-model.html"><strong aria-hidden="true">14.</strong> Data Model</a></li><li class="chapter-item "><a href="architecture/context-enrichment.html"><strong aria-hidden="true">15.</strong> Context Enrichment</a></li><li class="chapter-item "><a href="architecture/synapses.html"><strong aria-hidden="true">16.</strong> Conversation Threads (Synapses)</a></li><li class="chapter-item affix "><li class="part-title">Features</li><li class="chapter-item "><a href="features/providers.html"><strong aria-hidden="true">17.</strong> Multi-Provider Support</a></li><li class="chapter-item "><a href="features/token-management.html"><strong aria-hidden="true">18.</strong> Token Management</a></li><li class="chapter-item "><a href="features/partitioning.html"><strong aria-hidden="true">19.</strong> Partitioning &amp; Organization</a></li><li class="chapter-item "><a href="features/web-search.html"><strong aria-hidden="true">20.</strong> Web Search Integration</a></li><li class="chapter-item "><a href="features/import-export.html"><strong aria-hidden="true">21.</strong> Import/Export</a></li><li class="chapter-item affix "><li class="part-title">Deployment</li><li class="chapter-item "><a href="deployment/local.html"><strong aria-hidden="true">22.</strong> Local Development</a></li><li class="chapter-item "><a href="deployment/production.html"><strong aria-hidden="true">23.</strong> Production Setup</a></li><li class="chapter-item "><a href="deployment/environment.html"><strong aria-hidden="true">24.</strong> Environment Variables</a></li><li class="chapter-item affix "><li class="part-title">Troubleshooting</li><li class="chapter-item "><a href="troubleshooting/common-issues.html"><strong aria-hidden="true">25.</strong> Common Issues</a></li><li class="chapter-item "><a href="troubleshooting/faq.html"><strong aria-hidden="true">26.</strong> FAQ</a></li><li class="chapter-item "><a href="troubleshooting/debugging.html"><strong aria-hidden="true">27.</strong> Debugging</a></li><li class="chapter-item affix "><li class="part-title">Development</li><li class="chapter-item "><a href="development/contributing.html"><strong aria-hidden="true">28.</strong> Contributing</a></li><li class="chapter-item "><a href="development/building.html"><strong aria-hidden="true">29.</strong> Building from Source</a></li><li class="chapter-item "><a href="development/testing.html"><strong aria-hidden="true">30.</strong> Testing</a></li></ol>';
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
