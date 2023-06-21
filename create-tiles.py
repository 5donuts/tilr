#!/usr/bin/env python3

# create-tiles.py - A helper script for Tilr that creates test tiles
# Copyright (C) 2023  Charles German <5donuts@pm.me>
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

from PIL import Image
from pathlib import Path

# Equivalent to mkdir -p images/tiles/
Path("images/tiles/").mkdir(parents=True, exist_ok=True)

width = 250
height = 250
colors = [
    (0, 0, 0), # black
    (255, 255, 255), # white
    (208, 35, 35), # red
    (209, 136, 192), # pink
    (125, 36, 209), # purple
    (52, 36, 209), # blue
    (36, 167, 209), # lighter blue
    (36, 209, 136), # blue-ish green-ish
    (42, 209, 36), # green
    (159, 209, 36), # green-ish yellow-ish
    (209, 207, 36), # yellow
    (209, 108, 36), # orange
 ]

for i, c in enumerate(colors):
    img = Image.new(mode = "RGB", size = (width, height), color = c)
    img.save('images/tiles/tile-{0}.png'.format(i))
