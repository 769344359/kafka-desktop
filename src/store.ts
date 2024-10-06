import { createSlice, configureStore } from '@reduxjs/toolkit'

const counterSlice = createSlice({
  name: 'counter',
  initialState: {
    value: []
  },
  reducers: {
    setData: (state, newValue) => {
      console.log("value is is")
      console.log(newValue.payload)
        state.value = newValue.payload.filter( one => true)
    },
  }
})

export const { setData } = counterSlice.actions
const store = configureStore({
  reducer: counterSlice.reducer
});
export type SlotResource={
  slotNum:number,
  server:string,
  
}
export default store;

