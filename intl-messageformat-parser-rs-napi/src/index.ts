// From: https://github.com/formatjs/formatjs/blob/main/packages/intl-messageformat-parser/src/types.ts
export enum TYPE {
    /**
     * Raw text
     */
    literal,
    /**
     * Variable w/o any format, e.g `var` in `this is a {var}`
     */
    argument,
    /**
     * Variable w/ number format
     */
    number,
    /**
     * Variable w/ date format
     */
    date,
    /**
     * Variable w/ time format
     */
    time,
    /**
     * Variable w/ select format
     */
    select,
    /**
     * Variable w/ plural format
     */
    plural,
    /**
     * Only possible within plural argument.
     * This is the `#` symbol that will be substituted with the count.
     */
    pound,
    /**
     * XML-like tag
     */
    tag,
}

export const enum SKELETON_TYPE {
    number,
    dateTime,
}

export interface LocationDetails {
    offset: number;
    line: number;
    column: number;
}
export interface Location {
    start: LocationDetails;
    end: LocationDetails;
}

export interface BaseElement<T extends TYPE> {
    type: T;
    value: string;
    location?: Location;
}

export type LiteralElement = BaseElement<TYPE.literal>;
export type ArgumentElement = BaseElement<TYPE.argument>;
export interface TagElement {
    type: TYPE.tag;
    value: string;
    children: MessageFormatElement[];
    location?: Location;
}

export interface SimpleFormatElement<T extends TYPE, S extends Skeleton> extends BaseElement<T> {
    style?: string | S | null;
}

export type NumberElement = SimpleFormatElement<TYPE.number, NumberSkeleton>;
export type DateElement = SimpleFormatElement<TYPE.date, DateTimeSkeleton>;
export type TimeElement = SimpleFormatElement<TYPE.time, DateTimeSkeleton>;

export interface SelectOption {
    id: string;
    value: MessageFormatElement[];
    location?: Location;
}

export type ValidPluralRule = 'zero' | 'one' | 'two' | 'few' | 'many' | 'other' | string;

export interface PluralOrSelectOption {
    value: MessageFormatElement[];
    location?: Location;
}

export interface SelectElement extends BaseElement<TYPE.select> {
    options: Record<string, PluralOrSelectOption>;
}

export interface PluralElement extends BaseElement<TYPE.plural> {
    options: Record<ValidPluralRule, PluralOrSelectOption>;
    offset: number;
    pluralType: Intl.PluralRulesOptions['type'];
}

export interface PoundElement {
    type: TYPE.pound;
    location?: Location;
}

export type MessageFormatElement =
    | LiteralElement
    | ArgumentElement
    | NumberElement
    | DateElement
    | TimeElement
    | SelectElement
    | PluralElement
    | TagElement
    | PoundElement;

export interface NumberSkeletonToken {
    stem: string;
    options: string[];
}

export interface NumberSkeleton {
    type: SKELETON_TYPE.number;
    tokens: NumberSkeletonToken[];
    location?: Location;
    parsedOptions: Intl.NumberFormatOptions;
}

export interface DateTimeSkeleton {
    type: SKELETON_TYPE.dateTime;
    pattern: string;
    location?: Location;
    parsedOptions: Intl.DateTimeFormatOptions;
}

export type Skeleton = NumberSkeleton | DateTimeSkeleton;

import {loadBinding} from '@node-rs/helper';

const _parse = loadBinding(
    __dirname,
    'intl-messageformat-parser-rs',
    'intl-messageformat-parser-rs',
).parse;

export function parse(message: string): MessageFormatElement[] {
    return JSON.parse(_parse(message));
}
