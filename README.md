# img_rust

High-performance image processing library for Python, written in Rust with parallel processing capabilities.

## Overview

`img_rust` provides fast image blending operations for multi-dimensional image stacks, particularly useful for microscopy and scientific imaging applications. The library leverages Rust's performance and safety guarantees while providing a seamless Python interface through PyO3 and numpy integration.

## Features

- **Parallel Processing**: Process multiple image frames simultaneously using Rayon
- **Screen Blend Mode**: Implements photoshop-style screen blending for image compositing
- **4D Array Support**: Handles image stacks with dimensions (frames, height, width, channels)
- **Zero-Copy Integration**: Efficient data sharing between Python and Rust via numpy arrays
- **Memory Safe**: Leverages Rust's ownership system for safe memory management

## Installation

### Prerequisites

- Python 3.7+
- Rust (latest stable)
- numpy

### Build from source

```bash
git clone https://github.com/wl-stepp/img_rust.git
cd img_rust
pip install maturin
maturin develop
```

## Usage

```python
import numpy as np
import img_rust

# Create two image stacks (frames, height, width, channels)
stack1 = np.random.randint(0, 255, (10, 100, 100, 4), dtype=np.uint8)
stack2 = np.random.randint(0, 255, (10, 100, 100, 4), dtype=np.uint8)

# Apply screen blend mode across all frames in parallel
result = img_rust.screen_stack_py(stack1, stack2)
```

## Performance

The screen blending operation is parallelized across frames, providing significant speedup for large image stacks compared to pure Python implementations. Typical performance improvements of 5-20x have been observed depending on stack size and hardware.

## API Reference

### `screen_stack_py(stack1, stack2)`

Applies screen blend mode between two image stacks.

**Parameters:**
- `stack1` (numpy.ndarray): First image stack with shape (frames, height, width, 4)
- `stack2` (numpy.ndarray): Second image stack with shape (frames, height, width, 4)

**Returns:**
- `numpy.ndarray`: Blended image stack with same shape as inputs

**Note:** Both input arrays must have 4 channels (RGBA) and be of dtype `uint8`.

## Algorithm

The screen blend mode formula for each color channel is:
```
result = 255 - 255 * (1 - color1/255) * (1 - color2/255)
```

Alpha channel blending:
```
alpha = 255 - ((255 - alpha1) * (255 - alpha2) / 255)
```

## Use Cases

- **Microscopy**: Blending fluorescence channels from time-lapse imaging
- **Scientific Visualization**: Combining multiple data layers
- **Image Processing Pipelines**: High-performance batch processing
- **Research Applications**: Fast prototyping with performance-critical operations

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

MIT License - see LICENSE file for details.

## Author

Willi Stepp - [@wl-stepp](https://github.com/wl-stepp)
