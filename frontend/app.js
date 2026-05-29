// Function to switch between clean, isolated subscreen tabs
function goToTab(tabName) {
    // 1. Completely hide every layout view block
    const screens = document.querySelectorAll('.app-screen');
    screens.forEach(screen => screen.style.display = 'none');

    // 2. Wipe active status markers off footer items
    const navItems = document.querySelectorAll('.nav-item');
    navItems.forEach(item => item.classList.remove('active-nav'));

    // 3. Elements to control inside the top sticky panel
    const backBtn = document.getElementById('backBtn');
    const headerTitle = document.getElementById('headerTitle');
    const headerTagline = document.getElementById('headerTagline');
    const searchBox = document.getElementById('headerSearchBox');

    // 4. Isolated View Routing Logic paths
    if (tabName === 'home') {
        document.getElementById('homeScreen').style.display = 'block';
        document.getElementById('nav-home').classList.add('active-nav');
        headerTitle.innerText = "Rock Mkt";
        headerTagline.style.display = "block";
        searchBox.style.display = "flex";
        backBtn.style.display = "none";
    } 
    else if (tabName === 'friends') {
        document.getElementById('friendsScreen').style.display = 'block';
        document.getElementById('nav-friends').classList.add('active-nav');
        headerTitle.innerText = "Network Friends";
        headerTagline.style.display = "none";
        searchBox.style.display = "none";
        backBtn.style.display = "inline-flex";
    } 
    else if (tabName === 'messages') {
        document.getElementById('messagesScreen').style.display = 'block';
        document.getElementById('nav-messages').classList.add('active-nav');
        headerTitle.innerText = "Inbox Conversations";
        headerTagline.style.display = "none";
        searchBox.style.display = "none";
        backBtn.style.display = "inline-flex";
    } 
    else if (tabName === 'notifications') {
        document.getElementById('notificationsScreen').style.display = 'block';
        document.getElementById('nav-notifications').classList.add('active-nav');
        headerTitle.innerText = "Market Alerts";
        headerTagline.style.display = "none";
        searchBox.style.display = "none";
        backBtn.style.display = "inline-flex";
    } 
    else if (tabName === 'profile') {
        document.getElementById('profileScreen').style.display = 'block';
        document.getElementById('nav-profile').classList.add('active-nav');
        headerTitle.innerText = "My Studio Workspace";
        headerTagline.style.display = "none";
        searchBox.style.display = "none";
        backBtn.style.display = "inline-flex";
    }
}

// Sidebar open and close animation handler
function toggleSidebar(open) {
    const sidebar = document.getElementById('sidebarMenu');
    const overlay = document.getElementById('sidebarOverlay');
    
    if (open) {
        overlay.style.display = "block";
        setTimeout(() => {
            overlay.style.opacity = "1";
            sidebar.style.right = "0px";
        }, 10);
    } else {
        sidebar.style.right = "-280px";
        overlay.style.opacity = "0";
        setTimeout(() => { overlay.style.display = "none"; }, 300);
    }
}

// Operational toggle to handle Join button feeds
function handleJoinAction(cardId) {
    const targetButton = document.getElementById(`joinBtn${cardId}`);
    if (targetButton.classList.contains('joined-state')) return;
    targetButton.innerText = "Joined";
    targetButton.classList.add('joined-state');
}

// Core Product GPS Auto-Lock verification Permission Engine
function getLiveLocation() {
    const locationInput = document.getElementById('itemLocation');
    const nameInput = document.getElementById('itemName').value;
    const priceInput = document.getElementById('itemPrice').value;

    if (!nameInput || !priceInput) {
        alert("Error: Please provide a Product Name and baseline Price configuration first!");
        return;
    }

    if (!navigator.geolocation) {
        alert("Error: Hardware device does not support geolocation metrics.");
        return;
    }

    locationInput.placeholder = "Locating verified system coordinates...";

    navigator.geolocation.getCurrentPosition(
        (position) => {
            const lat = position.coords.latitude;
            const lng = position.coords.longitude;
            locationInput.value = `Lat: ${lat.toFixed(5)}, Lng: ${lng.toFixed(5)}`;
            alert(`Success! GPS Location Verified. Ready to launch post to the public marketplace feed!`);
        },
        (error) => {
            locationInput.placeholder = "Coordinates locked via active permissions";
            alert("Security Error: Active location telemetry handles tracking to block fraudulent listings.");
        },
        { enableHighAccuracy: true, timeout: 10000 }
    );
}

