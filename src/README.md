# Druid_Questions

A simple Druid demonstration to motivate a few questions.

The demo lets you select one of three boxes (red, green and blue) and adjust its size.

## My Questions

1. When ViewSwitcher changes views, how can I reset the Scale slider?
Currently it remains where it was in the previous view, making it difficult to fine tune the size of a box.

2. ctx.size() seems to be size of whole window. Probably because that's what layout() currently returns.

What should layout() return to create simple canvas that uses all available space?  If I do it right, will paintctx.size() give me the size of my widget's actual drawing region?

(In this demo I have to make ad hoc corrections in paint, based on what seems to work on my Mac.)

3. The BoxMaker widget needs to access state (BOXES) that persists between BoxMaker instances. Is there any alternative to lazy_static?

Maybe I could put a slider for each box in AppData, but I can't figure out how to make a Lens index into my array.  A single example of Index would be nice.

4. Is there a way to redefine the minimum window size?
