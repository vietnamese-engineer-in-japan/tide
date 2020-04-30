import * as mm from "@magenta/music";
import { saveAs } from "file-saver";
import * as wasm from "tide";

(async () => {
    const fileInputElement = document.getElementById("fileInput");
    const model = new mm.OnsetsAndFrames(
        "https://storage.googleapis.com/magentadata/js/checkpoints/transcription/onsets_frames_uni"
    );
    await model.initialize();
    fileInputElement.addEventListener("change", async _ => {
        const result = await wasm.read_file(fileInputElement.files[0]);
        const audio = await wasm.decode_audio(result);
        const rate = audio.sampleRate;
        const bufferLength = rate * 10;
        const data = new Float32Array(bufferLength);
        const context = new OfflineAudioContext(1, rate, rate);
        const buffer = context.createBuffer(2, bufferLength, rate);
        const sequences = [];
        for (let i = 0; i < Math.ceil(audio.length / bufferLength); i++) {
            for (let j = 0; j < audio.numberOfChannels; j++) {
                audio.copyFromChannel(data, j, i * bufferLength);
                buffer.copyToChannel(data, j, 0);
            }
            sequences.push(await model.transcribeFromAudioBuffer(buffer));
        }
        const sequence = mm.sequences.concatenate(sequences);
        saveAs(new File([mm.sequenceProtoToMidi(sequence)], "transcription.mid"));
    });
})();
