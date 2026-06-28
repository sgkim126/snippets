const background = document.querySelector("#background");
const canvasItems = document.querySelectorAll(".canvas-item");

background.addEventListener("click", () => {
  background.classList.remove("is-flashing");
  background.offsetWidth;
  background.classList.add("is-flashing");
});

function drawRotatingTetrahedron(canvasElement, colors) {
  const context = canvasElement.getContext("2d");
  let animationId = 0;
  let isPlaying = true;
  const vertices = [
    { x: 1, y: 1, z: 1 },
    { x: -1, y: -1, z: 1 },
    { x: -1, y: 1, z: -1 },
    { x: 1, y: -1, z: -1 },
  ];
  const faces = [
    { points: [0, 1, 2], color: colors[0] },
    { points: [0, 3, 1], color: colors[1] },
    { points: [0, 2, 3], color: colors[2] },
    { points: [1, 3, 2], color: colors[3] },
  ];

  function renderFrame(time) {
    const angle = time * 0.001;
    const projected = vertices.map((vertex) => (
      projectPoint(canvasElement, rotatePoint(vertex, angle))
    ));
    const sortedFaces = faces
      .map((face) => ({
        ...face,
        depth: face.points.reduce((sum, index) => sum + projected[index].z, 0) / face.points.length,
      }))
      .sort((a, b) => a.depth - b.depth);

    context.clearRect(0, 0, canvasElement.width, canvasElement.height);
    context.fillStyle = "black";
    context.fillRect(0, 0, canvasElement.width, canvasElement.height);

    sortedFaces.forEach((face) => {
      const [firstIndex, ...restIndexes] = face.points;
      const firstPoint = projected[firstIndex];

      context.beginPath();
      context.moveTo(firstPoint.x, firstPoint.y);
      restIndexes.forEach((index) => {
        context.lineTo(projected[index].x, projected[index].y);
      });
      context.closePath();
      context.fillStyle = face.color;
      context.fill();
      context.strokeStyle = "white";
      context.lineWidth = 1.5;
      context.stroke();
    });

    if (isPlaying) {
      animationId = requestAnimationFrame(renderFrame);
    }
  }

  function play() {
    if (isPlaying) {
      return;
    }

    isPlaying = true;
    animationId = requestAnimationFrame(renderFrame);
  }

  function pause() {
    isPlaying = false;
    cancelAnimationFrame(animationId);
  }

  animationId = requestAnimationFrame(renderFrame);

  return { play, pause };
}

function rotatePoint(point, angle) {
  const sinX = Math.sin(angle * 0.75);
  const cosX = Math.cos(angle * 0.75);
  const sinY = Math.sin(angle);
  const cosY = Math.cos(angle);
  const sinZ = Math.sin(angle * 0.45);
  const cosZ = Math.cos(angle * 0.45);

  let { x, y, z } = point;
  let nextY = y * cosX - z * sinX;
  let nextZ = y * sinX + z * cosX;
  y = nextY;
  z = nextZ;

  let nextX = x * cosY + z * sinY;
  nextZ = -x * sinY + z * cosY;
  x = nextX;
  z = nextZ;

  nextX = x * cosZ - y * sinZ;
  nextY = x * sinZ + y * cosZ;

  return { x: nextX, y: nextY, z };
}

function projectPoint(canvasElement, point) {
  const distance = 4;
  const scale = Math.min(canvasElement.width, canvasElement.height) * 0.29;
  const perspective = distance / (distance - point.z);

  return {
    x: canvasElement.width / 2 + point.x * scale * perspective,
    y: canvasElement.height / 2 + point.y * scale * perspective,
    z: point.z,
  };
}

const tetrahedronPalettes = [
  ["darkred", "crimson", "tomato", "salmon"],
  ["darkgreen", "seagreen", "limegreen", "palegreen"],
  ["navy", "royalblue", "deepskyblue", "lightskyblue"],
];
canvasItems.forEach((item, index) => {
  const canvasElement = item.querySelector(".tetrahedron-canvas");
  const controls = drawRotatingTetrahedron(canvasElement, tetrahedronPalettes[index]);
  const playButton = item.querySelector(".play-button");
  const pauseButton = item.querySelector(".pause-button");
  const growButton = item.querySelector(".grow-button");
  const shrinkButton = item.querySelector(".shrink-button");
  const sizeControls = item.querySelector(".size-controls");
  let startPointerX = 0;
  let startPointerY = 0;
  let startX = 0;
  let startY = 0;
  let currentX = 0;
  let currentY = 0;
  let canvasWidth = 300;
  let canvasHeight = 200;

  function clampValue(value, min, max) {
    if (min > max) {
      return (min + max) / 2;
    }

    return Math.min(Math.max(value, min), max);
  }

  function clampPosition(nextX, nextY) {
    const itemStyle = getComputedStyle(item);
    const baseX = parseFloat(itemStyle.getPropertyValue("--base-x")) || 0;
    const baseY = parseFloat(itemStyle.getPropertyValue("--base-y")) || 0;
    const halfWidth = item.offsetWidth / 2;
    const halfHeight = item.offsetHeight / 2;
    const topOverflow = index < 2 && !item.classList.contains("drag-area-includes-size-controls")
      ? sizeControls.offsetHeight
      : 0;

    return {
      x: clampValue(
        nextX,
        halfWidth - window.innerWidth / 2 - baseX,
        window.innerWidth / 2 - halfWidth - baseX
      ),
      y: clampValue(
        nextY,
        halfHeight + topOverflow - window.innerHeight / 2 - baseY,
        window.innerHeight / 2 - halfHeight - baseY
      ),
    };
  }

  function setPosition(nextX, nextY) {
    const nextPosition = clampPosition(nextX, nextY);

    currentX = nextPosition.x;
    currentY = nextPosition.y;
    item.style.setProperty("--x", `${currentX}px`);
    item.style.setProperty("--y", `${currentY}px`);
  }

  function resizeCanvas(nextWidth) {
    const previousWidth = item.offsetWidth;
    const previousHeight = item.offsetHeight;
    const nextHeight = Math.round(nextWidth * 2 / 3);
    const widthDelta = nextWidth - previousWidth;
    const nextItemHeight = item.classList.contains("drag-area-includes-size-controls")
      ? nextHeight + parseFloat(getComputedStyle(item).getPropertyValue("--size-control-height"))
      : nextHeight;
    const heightDelta = nextItemHeight - previousHeight;

    canvasWidth = nextWidth;
    canvasHeight = nextHeight;
    currentX -= widthDelta / 2;
    currentY += heightDelta / 2;
    canvasElement.width = canvasWidth;
    canvasElement.height = canvasHeight;
    item.style.setProperty("--canvas-width", `${canvasWidth}px`);
    item.style.setProperty("--canvas-height", `${canvasHeight}px`);
    setPosition(currentX, currentY);
  }

  playButton.addEventListener("pointerdown", (event) => {
    event.stopPropagation();
  });

  pauseButton.addEventListener("pointerdown", (event) => {
    event.stopPropagation();
  });

  growButton.addEventListener("pointerdown", (event) => {
    event.stopPropagation();
  });

  shrinkButton.addEventListener("pointerdown", (event) => {
    event.stopPropagation();
  });

  playButton.addEventListener("click", (event) => {
    event.stopPropagation();
    controls.play();
    item.classList.add("is-playing");
    item.classList.remove("is-paused");
  });

  pauseButton.addEventListener("click", (event) => {
    event.stopPropagation();
    controls.pause();
    item.classList.add("is-paused");
    item.classList.remove("is-playing");
  });

  growButton.addEventListener("click", (event) => {
    event.stopPropagation();
    resizeCanvas(Math.min(canvasWidth + 30, 540));
  });

  shrinkButton.addEventListener("click", (event) => {
    event.stopPropagation();
    resizeCanvas(Math.max(canvasWidth - 30, 150));
  });

  item.addEventListener("pointerdown", (event) => {
    startPointerX = event.clientX;
    startPointerY = event.clientY;
    startX = currentX;
    startY = currentY;
    item.classList.add("is-dragging");
    item.setPointerCapture(event.pointerId);
  });

  item.addEventListener("pointermove", (event) => {
    if (!item.hasPointerCapture(event.pointerId)) {
      return;
    }

    setPosition(
      startX + event.clientX - startPointerX,
      startY + event.clientY - startPointerY
    );
  });

  item.addEventListener("pointerup", (event) => {
    item.classList.remove("is-dragging");
    item.releasePointerCapture(event.pointerId);
  });

  item.addEventListener("pointercancel", (event) => {
    item.classList.remove("is-dragging");

    if (item.hasPointerCapture(event.pointerId)) {
      item.releasePointerCapture(event.pointerId);
    }
  });

  setPosition(currentX, currentY);

  window.addEventListener("resize", () => {
    setPosition(currentX, currentY);
  });
});

