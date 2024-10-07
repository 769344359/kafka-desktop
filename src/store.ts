import { createSlice, configureStore } from '@reduxjs/toolkit'
export type KafkaConfig ={
  topic:string,
  server:string,
};
export type ResourceConfig = {
  index:number,
  isDisabled:boolean,
  disabledKeys:string[]
}

let  resourceConfigSlice = createSlice({
  name: 'resourceConfigSlice',
  initialState: {
    value: [{index:0,isDisabled:false,disabledKeys:['topic','groups','consumer'],}] as ResourceConfig[]
  },
  reducers: {
    setData: (state, newValue) => {
      console.log("value is is")
      console.log(newValue.payload)
        state.value = newValue.payload
    },
  }
});
export const resourceStore = configureStore({
  reducer: resourceConfigSlice.reducer
});

const counterSlice = createSlice({
  name: 'counter',
  initialState: {
    value: []
  },
  reducers: {
    setData: (state, newValue) => {
      console.log("value is is")
      console.log(newValue.payload)
        state.value = newValue.payload
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

