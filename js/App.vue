<template>
  <div id="main_content">
    <three-js-example
    />
  </div>
</template>

<script>
import Vue from "vue";

import ThreeJsExample from './ThreeJsExample';

export default Vue.extend({
    components: {
        'three-js-example': ThreeJsExample
    },
  data() {
    return {
      bundler: "Parcel"
    };
  }
});
</script>

