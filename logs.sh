#!/bin/bash

tail -f ../logs/frontend.log &
tail -f ../logs/backend.log &