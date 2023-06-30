package org.cbtw;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class TestCases {

    @Test
    void testSimple() {
        final String key = "eljuxhpwnyrdgtqkviszcfmabo";
        final String encryptedMessage = "zwx hnfx lqantp mnoeius ycgk vcnjrdb";
        final String decryptedMessage = "the five boxing wizards jump quickly";

        final String result = new CaesarCipher().decipher(encryptedMessage, key);
        assertEquals(result, decryptedMessage);
    }

    @Test
    void testMoreComplex() {
        final String key = "the quick brown fox jumps over the lazy dog";
        final String encryptedMessage = "vkbs bs t suepuv";
        final String decryptedMessage = "this is a secret";

        final String result = new CaesarCipher().decipher(encryptedMessage, key);
        assertEquals(result, decryptedMessage);
    }

    @Test
    void testEdgeCase() {
        final String key = "the quick brown fox jumps over the lazy dog";
        final String encryptedMessage = "vkbs-bs-t-suepuv";
        final String decryptedMessage = "this-is-a-secret";

        final String result = new CaesarCipher().decipher(encryptedMessage, key);
        assertEquals(result, decryptedMessage);
    }

    @Test
    void testEmptyMessage() {
        final String key = "the quick brown fox jumps over the lazy dog";
        final String encryptedMessage = "  ";
        final String decryptedMessage = "  ";

        final String result = new CaesarCipher().decipher(encryptedMessage, key);
        assertEquals(result, decryptedMessage);
    }

}
