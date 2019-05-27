/* -*- Mode: C; indent-tabs-mode: nil; c-basic-offset: 4 -*- */
/* vim: set sw=4 sts=4 expandtab: */

#include <assert.h>
#include <stdio.h>
#include "bzlib_private.h"

static UInt32
crc32_buffer (const UChar *buf, size_t len)
{
    UInt32 state;
    size_t i;

    BZ2_initialise_crc (&state);

    for (i = 0; i < len; i++) {
        BZ2_update_crc (&state, buf[i]);
    }

    BZ2_finalise_crc (&state);

    return state;
}

static const UChar buf1[] = "";
static const UChar buf2[] = " ";
static const UChar buf3[] = "hello world";

static const UChar buf4[] =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, "
    "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. "
    "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi "
    "ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit "
    "in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur "
    "sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt "
    "mollit anim id est laborum.";

int
main (void)
{
    assert (crc32_buffer (buf1, strlen ((const char *) buf1)) == 0x00000000);
    assert (crc32_buffer (buf2, strlen ((const char *) buf2)) == 0x29d4f6ab);
    assert (crc32_buffer (buf3, strlen ((const char *) buf3)) == 0x44f71378);
    assert (crc32_buffer (buf4, strlen ((const char *) buf4)) == 0xd31de6c9);

    return 0;
}
