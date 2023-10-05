<template>
  <v-menu
    open-on-hover
    bottom
    offset-y
  >
    <template v-slot:activator="{ on }">
      <v-btn
        text
        dark
        :to="to"
        v-on="on"
      >
        <span>
          {{ text }}
        </span>
      </v-btn>
    </template>
    <v-list
      v-if="items"
    >
      <v-list-item
        v-for="(item, n) in items"
        :key="n"
        :to="item.to"
      >
        <v-list-item-title>
          {{ item.text }}
        </v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script lang="ts">
  import { Vue, Component, Prop } from 'vue-property-decorator'

  @Component({})
  export default class MenuButton extends Vue {
    @Prop() to!: string;
    @Prop() text!: string;
    @Prop() items!: string[];
  }
</script>
