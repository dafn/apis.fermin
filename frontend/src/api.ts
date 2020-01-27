interface Note {
  content: String;
  created: Date;
  id: number;
}

let mock: Note[] = [
  {
    id: 1,
    content: "Note 1",
    created: new Date()
  },
  {
    id: 2,
    content: "Note 2",
    created: new Date()
  }
];

const getAllNotes = () => {
  return mock;
};

const getNoteById = (id: number) => {
  try {
    return mock[id];
  } catch {
    return false;
  }
};

const deleteAllNotes = () => {
  mock = [];
};

const deleteNoteById = (id: number) => {
  try {
    delete mock[id];
    return true;
  } catch {
    return false;
  }
};

const postNote = (note: Note) => {
  try {
    mock.push(note);
    return true;
  } catch {
    return false;
  }
};

export { getAllNotes, getNoteById, deleteAllNotes, deleteNoteById, postNote };
