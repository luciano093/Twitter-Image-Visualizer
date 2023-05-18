const sidebarToggleBtn = document.getElementById("sidebar-toggle-btn");
const sidebar = document.querySelector(".sidebar");

let sidebarOpen = false;

sidebarToggleBtn.addEventListener("click", () => {
    sidebarOpen = !sidebarOpen;

    if (sidebarOpen) {
        contentContainer.style.marginLeft = `${sidebar.offsetWidth}px`;
        sidebar.style.left = "0";
    } else {
        contentContainer.style.marginLeft = "0";
        sidebar.style.left = `-${sidebar.offsetWidth}px`;
    }
});

const banner = document.getElementById("header");

// Calculate the height of the banner, including margins
const bannerHeight = banner.offsetHeight + parseInt(getComputedStyle(banner).marginTop) + parseInt(getComputedStyle(banner).marginBottom);



const sideBarText = document.getElementById("sidebar-text");


// Set the sidebar's top position equal to the banner's height
sidebar.style.top = `${bannerHeight}px`;

// Make images appear below sidebar
const contentContainer = document.querySelector(".content-container");
// const image = contentContainer.firstChild.firstChild;
// console.log(image.style.marginTop)

contentContainer.style.marginTop = `${bannerHeight * 1.1}px`;
contentContainer.style.marginLeft = "0";
sidebar.style.top = `${bannerHeight}px`;