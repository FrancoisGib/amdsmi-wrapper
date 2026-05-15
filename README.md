Required (no bindgen):
```bash
sudo apt install amd-smi-lib
```

required to generate bindings:
```bash
sudo apt install clang libclang-dev llvm pkg-config
```

generate bindings:
```bash
# use env variable AMDSMI_LIB_DIR to specify a lib dir
AMDSMI_GENERATE_BINDINGS= cargo build
```

if error linking libamd_smi:
```bash
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/opt/rocm/lib # or your rocm path
```