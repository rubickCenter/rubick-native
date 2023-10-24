import test from 'ava';
import { getClipboardContent, ClipboardContent } from '../src'

test('get clipboard content', (t) => {
  const result: ClipboardContent = getClipboardContent()
  t.true((result.type === 'text' && typeof result.content === 'string') || (result.type === 'file' && typeof result.content === 'object') || result.type === null)
});
