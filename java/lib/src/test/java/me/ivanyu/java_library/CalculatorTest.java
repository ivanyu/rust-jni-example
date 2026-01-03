package me.ivanyu.java_library;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

class CalculatorTest {
    @Test
    void testAdd() {
        final Result[] capturedResult = new Result[1];
        Calculator.add(2, 3, result -> capturedResult[0] = result);
        
        assertNotNull(capturedResult[0]);
        assertEquals("Result: 5", capturedResult[0].getMessage());
    }
}
