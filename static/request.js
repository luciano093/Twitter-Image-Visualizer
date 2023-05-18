let currentImageIndex = null;
let totalLoadedMedia = 0; // used to give each image its index
let imageWidth = 200;
let imageHeight = 270;

function fetchJSON(url){
    return new Promise((resolve, reject) => {
        let xmlHttp = new XMLHttpRequest();
        xmlHttp.onreadystatechange = () => { 
            if (xmlHttp.readyState == 4) {
                if (xmlHttp.status == 200) {
                  resolve(JSON.parse(xmlHttp.responseText));
                } else {
                  reject(new Error(`Request failed with status ${xmlHttp.status}`));
                }
              }
        }

        xmlHttp.open("GET", url, true); // true for asynchronous

        xmlHttp.send(null);
    });
}


function openImageModal(imgSrc, index) {
    const modal = document.getElementById("image-modal");
    const modalImage = document.getElementById("modal-image");

    modal.style.display = "flex";
    modalImage.src = imgSrc;
    currentImageIndex = index;

    const closeBtn = document.querySelector(".close");
    closeBtn.onclick = () => {
        modal.style.display = "none";
    };

    window.onclick = (event) => {
        if (event.target === modal) {
            modal.style.display = "none";
            modalImage.src = null;
        }
    };
}

async function loadImages(requestCount, mediaCount, cursor) {
    const imageContainer = document.getElementById("image-container");

    for (let i = 0; i < requestCount; i++) {
        const query = `?count=${mediaCount}${cursor ? `&cursor="${encodeURIComponent(cursor)}"` : ''}`;
        const requestUrl = `/i/api/media${window.location.pathname}${query}`;

        try {
            const json = await fetchJSON(requestUrl);

            if (json.error) {
                console.error("error: " + json.error.message);
                return;
            }

            const mediaArray = json.media;

            let mediaRecieved = 0;

            mediaArray.forEach((media, index) => {
                const imgSrc = `${media}?name=small`;
                const img = new Image();
                img.src = imgSrc;
                img.width = imageWidth;
                img.height = imageHeight;
                img.style = "object-fit: cover";
                img.index = totalLoadedMedia;

                img.onclick = () => {
                    openImageModal(imgSrc.replace("name=small", "name=large"), img.index); // Replace with the URL for the larger image
                };

                imageContainer.appendChild(img);
                totalLoadedMedia++;
            });
        
            if (json.cursor) {
                cursor = json.cursor;
            }
        } catch (error) {
            console.error(`Error fetching data: ${error}`)
        }
    }

    return cursor;
}

function switchImage(direction) {
    const modal = document.getElementById("image-modal");

    if(!modal.style.display || modal.style.display === "none") return;

    const imageContainer = document.getElementById("image-container");
    const images = Array.from(imageContainer.children);

    if (direction === "prev" && currentImageIndex > 0) {
        currentImageIndex--;
    } else if (direction === "next" && currentImageIndex < images.length - 1) {
        currentImageIndex++;
    }

    const imgSrc = images[currentImageIndex].src.replace("name=small", "name=large");
    const modalImage = document.getElementById("modal-image");
    modalImage.src = imgSrc;
}

function closeModal() {
    const modal = document.getElementById("image-modal");

    if(!modal.style.display || modal.style.display === "none") return;

    modal.style.display = "none";

    const modalImage = document.getElementById("modal-image");
    modalImage.src = null;
}

document.addEventListener("keydown", (event) => {
    if (event.key === "ArrowLeft") {
        switchImage("prev");
    } else if (event.key === "ArrowRight") {
        switchImage("next");
    } else if (event.key === "Escape") {
        closeModal();
    }
});

const main = async () => {
    let screenMaxImageNumber = () => Math.floor((window.innerHeight / imageHeight) * (window.innerWidth / imageWidth) * 1.3);

    let cursor = await loadImages(1, screenMaxImageNumber());
    let isLoading = false;

    await window.addEventListener('scroll', async () => {
        if (!isLoading && (window.innerHeight + window.scrollY >= document.body.offsetHeight * 0.90)) {
            isLoading = true;
            cursor = await loadImages(1, screenMaxImageNumber(), cursor);
            isLoading = false;
        }
    });
}

if (window.location.pathname != "/home" &&
    window.location.pathname != "" &&
    window.location.pathname != "/"
){
    main();
}