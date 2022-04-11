#!/bin/bash

for filename in ./*; do
    new=$(echo $filename | sed -e 's/\.\/\(0*\)//g')
    mv $filename $new

done
