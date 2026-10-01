use crate::prelude::*;

pub type __int64_t = c_longlong;
pub type __uint64_t = c_ulonglong;

pub type int_fast16_t = c_int;
pub type int_fast32_t = c_int;
pub type int_fast64_t = c_longlong;
pub type uint_fast16_t = c_uint;
pub type uint_fast32_t = c_uint;
pub type uint_fast64_t = c_ulonglong;

pub type __quad_t = c_longlong;
pub type __u_quad_t = c_ulonglong;
pub type __intmax_t = c_longlong;
pub type __uintmax_t = c_ulonglong;

pub type __squad_type = __int64_t;
pub type __uquad_type = __uint64_t;
pub type __sword_type = c_int;
pub type __uword_type = c_uint;
pub type __slong32_type = c_long;
pub type __ulong32_type = c_ulong;
pub type __s64_type = __int64_t;
pub type __u64_type = __uint64_t;

pub type __ipc_pid_t = c_ushort;

pub type Elf32_Half = u16;
pub type Elf32_Word = u32;
pub type Elf32_Off = u32;
pub type Elf32_Addr = u32;
pub type Elf32_Section = u16;

pub type Elf_Addr = crate::Elf32_Addr;
pub type Elf_Half = crate::Elf32_Half;
pub type Elf_Ehdr = crate::Elf32_Ehdr;
pub type Elf_Phdr = crate::Elf32_Phdr;
pub type Elf_Shdr = crate::Elf32_Shdr;
pub type Elf_Sym = crate::Elf32_Sym;

// sys/ucontext.h
pub type greg_t = c_int;
pub type gregset_t = [greg_t; 19usize];

s! {
    pub struct Elf32_Ehdr {
        pub e_ident: [c_uchar; 16],
        pub e_type: Elf32_Half,
        pub e_machine: Elf32_Half,
        pub e_version: Elf32_Word,
        pub e_entry: Elf32_Addr,
        pub e_phoff: Elf32_Off,
        pub e_shoff: Elf32_Off,
        pub e_flags: Elf32_Word,
        pub e_ehsize: Elf32_Half,
        pub e_phentsize: Elf32_Half,
        pub e_phnum: Elf32_Half,
        pub e_shentsize: Elf32_Half,
        pub e_shnum: Elf32_Half,
        pub e_shstrndx: Elf32_Half,
    }

    pub struct Elf32_Shdr {
        pub sh_name: Elf32_Word,
        pub sh_type: Elf32_Word,
        pub sh_flags: Elf32_Word,
        pub sh_addr: Elf32_Addr,
        pub sh_offset: Elf32_Off,
        pub sh_size: Elf32_Word,
        pub sh_link: Elf32_Word,
        pub sh_info: Elf32_Word,
        pub sh_addralign: Elf32_Word,
        pub sh_entsize: Elf32_Word,
    }

    pub struct Elf32_Sym {
        pub st_name: Elf32_Word,
        pub st_value: Elf32_Addr,
        pub st_size: Elf32_Word,
        pub st_info: c_uchar,
        pub st_other: c_uchar,
        pub st_shndx: Elf32_Section,
    }

    pub struct Elf32_Phdr {
        pub p_type: crate::Elf32_Word,
        pub p_offset: crate::Elf32_Off,
        pub p_vaddr: crate::Elf32_Addr,
        pub p_paddr: crate::Elf32_Addr,
        pub p_filesz: crate::Elf32_Word,
        pub p_memsz: crate::Elf32_Word,
        pub p_flags: crate::Elf32_Word,
        pub p_align: crate::Elf32_Word,
    }

    // sys/ucontext.h
    pub struct __c_anonymous_fpchip_state {
        pub state: [c_int; 27],
        pub status: c_int,
    }

    pub struct __c_anonymous_fp_emul_space {
        pub fp_emul: [c_char; 246],
        pub epad: [c_char; 2],
    }

    /// Note that unlike the x86-64 port, `fpregs` is held by value, not as a pointer.
    pub struct fpregset_t {
        pub fp_reg_set: __c_anonymous_fp_reg_set,
        pub f_wregs: [c_long; 33],
    }

    pub struct mcontext_t {
        pub gregs: crate::gregset_t,
        pub fpregs: fpregset_t,
    }

    /// Note that `uc_sigmask` precedes `uc_stack` here, unlike the x86-64 port.
    pub struct ucontext_t {
        pub uc_flags: c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_sigmask: crate::sigset_t,
        pub uc_stack: crate::stack_t,
        pub uc_mcontext: mcontext_t,
        __glibc_reserved1: Padding<[c_long; 5]>,
    }

    // sys/ipc.h
    pub struct ipc_perm {
        pub __key: crate::key_t,
        pub uid: c_ushort,
        pub gid: c_ushort,
        pub cuid: c_ushort,
        pub cgid: c_ushort,
        pub mode: c_ushort,
        pub __seq: c_ushort,
    }
}

s_no_extra_traits! {
    pub union __c_anonymous_fp_reg_set {
        pub fpchip_state: __c_anonymous_fpchip_state,
        pub fp_emul_space: __c_anonymous_fp_emul_space,
        pub f_fpregs: [c_int; 62],
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for __c_anonymous_fp_reg_set {
            fn eq(&self, other: &__c_anonymous_fp_reg_set) -> bool {
                unsafe {
                    self.fpchip_state == other.fpchip_state
                        && self.fp_emul_space == other.fp_emul_space
                        && self.f_fpregs == other.f_fpregs
                }
            }
        }
        impl Eq for __c_anonymous_fp_reg_set {}
        impl hash::Hash for __c_anonymous_fp_reg_set {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                unsafe {
                    self.fpchip_state.hash(state);
                    self.fp_emul_space.hash(state);
                    self.f_fpregs.hash(state);
                }
            }
        }
    }
}
