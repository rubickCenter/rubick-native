import test from 'ava';
import { keyboard, Key, mouse, left, right, up, down } from '../src'

test('simulateKeyboardTap one key', (t) => {
  t.notThrows(async () => await keyboard.type('a'))
});

test('simulateKeyboardTap multi key', (t) => {
  t.notThrows(async () => await keyboard.type('calculator'))
});

test('simulateKeyboardTap key with modifier', (t) => {
  t.notThrows(async () => await keyboard.type(Key.LeftControl, Key.LeftSuper, Key.A))
});

test('mouse move', (t) => {
  t.notThrows(async () => await mouse.move(left(100)))
  t.notThrows(async () => await mouse.move(up(50)))
  t.notThrows(async () => await mouse.move(right(100)))
  t.notThrows(async () => await mouse.move(down(50)))
});
