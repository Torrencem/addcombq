#!/usr/bin/env bash

for f in test_*.py
do
    sage "$f" -v
done