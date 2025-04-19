#ifdef __FreeBSD__
#include <sys/types.h>
#include <sys/param.h>
#include <sys/module.h>
#include <sys/kernel.h>
#include <sys/systm.h>
#include <sys/conf.h>
#include <sys/vnode.h>
#include <sys/mount.h>
#include <sys/namei.h>
#include <sys/file.h>
#else

typedef int c_int;
typedef void c_void;
#define MOD_LOAD 1
#define MOD_UNLOAD 2
#define EOPNOTSUPP 45
#define ENXIO 6
#define EBUSY 16

typedef struct module {
    int dummy;
} module;

typedef struct moduledata {
    const char* name;
    int (*evhand)(struct module*, int, void*);
    void* priv;
} moduledata_t;


typedef struct vnode {
    void* v_op;
} vnode;

typedef struct vop_read_args {
    struct vnode* a_vp;
    void* a_uio;
    int a_ioflag;
    void* a_cred;
} vop_read_args;

typedef struct vop_write_args {
    struct vnode* a_vp;
    void* a_uio;
    int a_ioflag;
    void* a_cred;
} vop_write_args;

typedef struct uio {
    int uio_resid;
} uio;
#endif