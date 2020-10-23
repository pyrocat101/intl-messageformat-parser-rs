import benchmark from 'benchmark';
import baselineParser from 'intl-messageformat-parser';
import * as parser from '../pkg/intl_messageformat_parser_wasm.js';

const complexMsg =
    '' +
    '{gender_of_host, select, ' +
    'female {' +
    '{num_guests, plural, offset:1 ' +
    '=0 {{host} does not give a party.}' +
    '=1 {{host} invites <em>{guest}</em> to her party.}' +
    '=2 {{host} invites <em>{guest}</em> and <em>one</em> other person to her party.}' +
    'other {{host} invites <em>{guest}</em> and <em>#</em> other people to her party.}}}' +
    'male {' +
    '{num_guests, plural, offset:1 ' +
    '=0 {{host} does not give a party.}' +
    '=1 {{host} invites <em>{guest}</em> to his party.}' +
    '=2 {{host} invites <em>{guest}</em> and one other person to his party.}' +
    'other {{host} invites <em>{guest}</em> and <em>#</em> other people to his party.}}}' +
    'other {' +
    '{num_guests, plural, offset:1 ' +
    '=0 {{host} does not give a party.}' +
    '=1 {{host} invites <em>{guest}</em> to their party.}' +
    '=2 {{host} invites <em>{guest}</em> and one other person to their party.}' +
    'other {{host} invites <em>{guest}</em> and <em>#</em> other people to their party.}}}}';

const normalMsg =
    '' +
    'Yo, {firstName} {lastName} has ' +
    '{numBooks, number, integer} ' +
    '{numBooks, plural, ' +
    'one {book} ' +
    'other {books}}.';

const simpleMsg = 'Hello, {name}!';

const stringMsg = 'Hello, world!';

console.log('Baseline:');
new benchmark.Suite()
    .add('complex_msg', () => baselineParser.parse(complexMsg))
    .add('normal_msg', () => baselineParser.parse(normalMsg))
    .add('simple_msg', () => baselineParser.parse(simpleMsg))
    .add('string_msg', () => baselineParser.parse(stringMsg))
    .on('cycle', (event) => {
        console.log(String(event.target));
    })
    .run();

console.log('Current:');
new benchmark.Suite()
    .add('complex_msg', () => parser.parse(complexMsg))
    .add('normal_msg', () => parser.parse(normalMsg))
    .add('simple_msg', () => parser.parse(simpleMsg))
    .add('string_msg', () => parser.parse(stringMsg))
    .on('cycle', (event) => {
        console.log(String(event.target));
    })
    .run();
