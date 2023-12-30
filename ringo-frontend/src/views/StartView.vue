<script setup lang="ts">
import { ref } from "vue";
import {
   NForm,
   NFormItem,
   NInput,
   NButton,
   type FormInst,
   FormItemInst,
   FormItemRule,
   FormValidationError,
   useMessage,
   FormRules, NImage
} from "naive-ui";

const formRef = ref<FormInst | null>(null);
const message = useMessage();
const formValue = ref({
   user: {
      name: "",
      password: "",
   },
});
const rules = ref({
   user: {
      name: {
         required: true,
         message: "Please input your nickname",
         trigger: "blur",
      },
      password: {
         required: true, // TODO: check this (or just rework this password thingy in general)
         message: "Please input the room password",
         trigger: ["input", "blur"],
      },
   },
});

function handleValidateClick(e: MouseEvent) {
   e.preventDefault();
   formRef.value?.validate((errors) => {
      if (!errors) {
         message.success("Valid");
      } else {
         console.log(errors);
         message.error("Invalid");
      }
   });
}
</script>

<template>
   <div class="flex h-screen justify-center align-middle">
      <div class="self-center flex flex-col gap-8">
         <div class="branding flex flex-col gap-2 text-center">
            <n-image class="w-1/2 self-center" src="/src/assets/ringo-logo.svg" alt="Ringo Logo" preview-disabled />
            <h1 class="text-7xl">RINGO</h1>
         </div>
         <n-form :model="formValue" ref="formRef" label-placement="top" class="form flex flex-col gap-2" :rules="rules">
            <n-form-item label="Nickname" path="nicknameValue">
               <n-input v-model:value="formValue.user.name" placeholder="Nickname" />
            </n-form-item>
            <n-form-item label="Password" path="passwordValue">
               <n-input v-model:value="formValue.user.password" placeholder="Password" type="password" show-password-on="click" />
            </n-form-item>
            <n-button
               :disabled="formValue.user.name == null || formValue.user.password == null"
               round
               @click="handleValidateClick"
            >
               Join Room
            </n-button>
         </n-form>
      </div>
   </div>
</template>

<style scoped lang="scss">
</style>
